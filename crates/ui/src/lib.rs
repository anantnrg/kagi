pub mod app;
pub mod assets;
pub mod control_bar;
mod keybinds;
pub mod layout;
pub mod main_view;
pub mod player_context;
pub mod res_handler;
pub mod sidebar;
pub mod titlebar;

use app::Kagi;
use assets::*;
use backend::{
    Backend,
    playback::{Playlist, SavedPlaylists},
    player::{Controller, Player, Response},
};
use components::{
    slider::{Slider, SliderEvent},
    theme::Theme,
};
use control_bar::ControlBar;
use gpui::*;
use gstreamer::State;
use layout::Layout;
use main_view::MainView;
use player_context::{PlayerContext, PlayerStateEvent, Thumbnail, Track};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use res_handler::ResHandler;
use sidebar::{LeftSidebar, RightSidebar};
use souvlaki::{
    MediaControlEvent, MediaControls, MediaMetadata, MediaPlayback, PlatformConfig, SeekDirection,
};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use titlebar::Titlebar;

actions!(
    text_input,
    [
        Backspace,
        Delete,
        Left,
        Right,
        SelectLeft,
        SelectRight,
        SelectAll,
        Home,
        End,
        Paste,
        Cut,
        Copy,
    ]
);

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let assets = Assets {};
    let app = Application::new().with_assets(assets.clone());

    app.run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(720.0)), cx);
        assets.load_fonts(cx).expect("Could not load fonts");
        components::input::bind_actions(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some(String::from("kagi")),
                focus: true,
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    ..Default::default()
                }),
                window_min_size: Some(Size {
                    width: px(400.0),
                    height: px(400.0),
                }),
                ..Default::default()
            },
            |win, cx| {
                cx.new(|cx| {
                    let player_context = PlayerContext::new(cx);
                    let res_handler = cx.new(|_| ResHandler {});
                    let arc_res = Arc::new(res_handler.clone());
                    // fix this
                    let mut hwnd: Option<*mut std::ffi::c_void> = None;
                    let handle = win.window_handle().unwrap().as_raw();
                    match handle {
                        RawWindowHandle::Win32(win32) => {
                            hwnd = Some(win32.hwnd.get() as *mut std::ffi::c_void);
                        }
                        _ => {}
                    }
                    let (mut player, controller) =
                        Player::new(backend.clone(), Arc::new(Mutex::new(Playlist::default())));

                    controller.load_theme();
                    cx.set_global(Theme::default());

                    let theme = cx.global::<Theme>().clone();

                    let vol_slider = cx.new(|_| {
                        Slider::new()
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.2)
                            .bg(theme.control_bar.volume_bg)
                            .fill(theme.control_bar.volume_fill)
                            .thumb_bg(theme.control_bar.volume_thumb)
                    });
                    let playbar = cx.new(|_| {
                        Slider::new()
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.0)
                            .bg(theme.control_bar.playbar_bg)
                            .fill(theme.control_bar.playbar_fill)
                            .thumb_bg(theme.control_bar.playbar_thumb)
                    });
                    let layout = Layout::new(cx);
                    let recv_controller = controller.clone();
                    let saved_playlists = cx.new(|_| SavedPlaylists::default());
                    let playlists = saved_playlists.clone();

                    keybinds::register(cx);
                    cx.set_global(controller);
                    cx.set_global(player_context.clone());
                    cx.set_global(layout);
                    cx.background_executor()
                        .spawn(async move {
                            player.run().await;
                        })
                        .detach();
                    let config = PlatformConfig {
                        dbus_name: "kagi",
                        display_name: "Kagi",
                        hwnd,
                    };
                    let mut controls = MediaControls::new(config).unwrap();
                    controls
                        .attach({
                            let controller = cx.global::<Controller>().clone();
                            let current_pos =
                                cx.global::<PlayerContext>().state.read(cx).position.clone();
                            let total_duration = cx
                                .global::<PlayerContext>()
                                .metadata
                                .read(cx)
                                .duration
                                .clone();
                            move |event: MediaControlEvent| match event {
                                MediaControlEvent::Play => {
                                    controller.play();
                                }
                                MediaControlEvent::Pause => {
                                    controller.pause();
                                }
                                MediaControlEvent::Previous => {
                                    controller.prev();
                                }
                                MediaControlEvent::Next => {
                                    controller.next();
                                }
                                MediaControlEvent::SeekBy(direction, duration) => {
                                    let seek_amount = duration.as_secs() as u64;

                                    let new_position = match direction {
                                        SeekDirection::Forward => {
                                            (current_pos + seek_amount).clamp(0, total_duration)
                                        }
                                        SeekDirection::Backward => {
                                            current_pos.saturating_sub(seek_amount)
                                        }
                                    };

                                    controller.seek(new_position);
                                }
                                _ => {}
                            }
                        })
                        .unwrap();

                    cx.spawn(|_, cx: AsyncApp| async move {
                        let res_handler = arc_res.clone();
                        loop {
                            while let Ok(res) = recv_controller.rx.try_recv() {
                                res_handler
                                    .update(&mut cx.clone(), |res_handler, cx| {
                                        res_handler.handle(cx, res);
                                    })
                                    .expect("Could not update");
                            }
                            cx.background_executor()
                                .timer(Duration::from_millis(10))
                                .await;
                        }
                    })
                    .detach();
                    cx.subscribe(
                        &vol_slider,
                        move |_: &mut Kagi, _, event: &SliderEvent, cx| match event {
                            SliderEvent::Change(vol) => {
                                let volume = (vol * 100.0).round() as f64 / 100.0;
                                let state = cx.global_mut::<PlayerContext>().state.clone();
                                state.update(cx, |this, cx| {
                                    this.volume = volume.clone();
                                    cx.notify();
                                });
                                cx.global::<Controller>().volume(volume);

                                cx.notify();
                            }
                        },
                    )
                    .detach();
                    cx.subscribe(&playbar, move |_: &mut Kagi, _, event: &SliderEvent, cx| {
                        match event {
                            SliderEvent::Change(time) => {
                                let state_write = cx.global_mut::<PlayerContext>().state.clone();
                                let controller = cx.global::<Controller>();
                                let meta = cx.global::<PlayerContext>().metadata.read(cx);
                                let total_duration = meta.duration as f32;
                                if total_duration > 0.0 {
                                    let seek_time =
                                        (total_duration * (*time as f32)).round() as u64;
                                    controller.seek(seek_time);
                                    state_write.update(cx, |this, cx| {
                                        this.position = seek_time;
                                        cx.notify();
                                    });
                                }

                                cx.notify();
                            }
                        }
                    })
                    .detach();
                    let vol_slider_clone = vol_slider.clone();
                    cx.subscribe(
                        &player_context.state,
                        move |_: &mut Kagi, _, event: &PlayerStateEvent, cx| match event {
                            PlayerStateEvent::Volume(vol) => {
                                vol_slider_clone.update(cx, |this, cx| {
                                    this.value(*vol as f32, cx);
                                });
                                cx.notify();
                            }
                        },
                    )
                    .detach();

                    let playbar_clone = playbar.clone();
                    let vol_slider_clone = vol_slider.clone();
                    let controls_arc = Arc::new(Mutex::new(controls));
                    cx.subscribe(
                        &res_handler,
                        move |_: &mut Kagi, _, event: &Response, cx| match event {
                            Response::Eos => {
                                if cx.global::<PlayerContext>().state.read(cx).repeat {
                                    cx.global::<Controller>().seek(0);
                                } else {
                                    cx.global::<Controller>().next();
                                }
                            }
                            Response::Position(pos) => {
                                let state = cx.global_mut::<PlayerContext>().state.clone();
                                state.update(cx, |state, cx| {
                                    state.position = *pos;
                                    cx.notify();
                                });
                                let duration = cx
                                    .global::<PlayerContext>()
                                    .metadata
                                    .read(cx)
                                    .duration
                                    .clone();
                                let slider_value = (*pos as f64 / duration as f64) as f32;
                                playbar_clone.update(cx, |this, cx| {
                                    this.value(slider_value, cx);
                                });
                                cx.notify();
                            }
                            Response::StreamStart => cx.global::<Controller>().get_meta(),
                            Response::Metadata(track) => {
                                let metadata = cx.global_mut::<PlayerContext>().metadata.clone();
                                let controls = controls_arc.clone();
                                metadata.update(cx, |meta, cx| {
                                    let track = track.clone();
                                    meta.title = track.title.clone().into();
                                    meta.album = track.album.clone().into();
                                    meta.artists = track
                                        .artists
                                        .clone()
                                        .iter()
                                        .map(|s| s.clone().into())
                                        .collect();
                                    meta.duration = track.clone().duration;
                                    cx.notify();
                                    controls
                                        .lock()
                                        .unwrap()
                                        .set_metadata(MediaMetadata {
                                            title: Some(track.title.to_string().as_str()),
                                            artist: Some(
                                                track.artists.join(", ").to_string().as_str(),
                                            ),
                                            album: Some(track.album.to_string().as_str()),
                                            duration: Some(Duration::from_secs(meta.duration)),
                                            ..Default::default()
                                        })
                                        .unwrap();
                                });
                            }
                            Response::Thumbnail(thumbnail) => {
                                let metadata = cx.global_mut::<PlayerContext>().metadata.clone();
                                metadata.update(cx, |meta, cx| {
                                    meta.thumbnail = Some(Thumbnail {
                                        img: ImageSource::Render(
                                            RenderImage::new(thumbnail.clone().to_frame()).into(),
                                        ),
                                        width: thumbnail.width,
                                        height: thumbnail.height,
                                    });
                                    cx.notify();
                                });
                            }
                            Response::StateChanged(new_state) => {
                                let state = cx.global_mut::<PlayerContext>().state.clone();
                                state.update(cx, |state, cx| {
                                    state.state = new_state.clone();
                                    cx.notify();
                                });
                                let playback_status = match new_state {
                                    State::Playing => MediaPlayback::Playing { progress: None },
                                    State::Paused => MediaPlayback::Paused { progress: None },
                                    _ => MediaPlayback::Stopped,
                                };
                                controls_arc
                                    .lock()
                                    .unwrap()
                                    .set_playback(playback_status)
                                    .expect("Could not set playback state");
                            }
                            Response::Tracks(new_tracks) => {
                                let tracks = cx.global_mut::<PlayerContext>().tracks.clone();
                                tracks.update(cx, |tracks, cx| {
                                    let mut np_tracks = vec![];
                                    for track in new_tracks {
                                        if let Some(thumbnail) = track.thumbnail.clone() {
                                            np_tracks.push(Track {
                                                album: track.album.clone(),
                                                artists: track.artists.clone(),
                                                duration: track.duration,
                                                thumbnail: Some(Thumbnail {
                                                    img: ImageSource::Render(
                                                        RenderImage::new(thumbnail.to_frame())
                                                            .into(),
                                                    ),
                                                    width: thumbnail.width,
                                                    height: thumbnail.height,
                                                }),
                                                title: track.title.clone(),
                                                uri: track.uri.clone(),
                                            });
                                        }
                                    }
                                    *tracks = np_tracks;
                                    cx.notify();
                                });
                            }
                            Response::SavedPlaylists(playlists) => {
                                saved_playlists.update(cx, |this, cx| {
                                    *this = playlists.clone();
                                    cx.notify();
                                });
                                cx.refresh_windows();
                            }
                            Response::PlaylistName(name) => {
                                let meta = cx.global_mut::<PlayerContext>().metadata.clone();
                                meta.update(cx, |meta, cx| {
                                    meta.playlist_name = name.clone().into();
                                    cx.notify();
                                });
                            }
                            Response::Shuffle(shuffle) => {
                                let state = cx.global_mut::<PlayerContext>().state.clone();
                                state.update(cx, |state, cx| {
                                    state.shuffle = shuffle.clone();
                                    cx.notify();
                                });
                            }
                            Response::Theme(theme) => {
                                cx.set_global::<Theme>(theme.clone().into());
                                cx.refresh_windows();
                            }
                            Response::Volume(vol) => {
                                vol_slider_clone.update(cx, |this, cx| {
                                    this.value(*vol as f32, cx);
                                });
                                let state = cx.global_mut::<PlayerContext>().state.clone();
                                state.update(cx, |state, cx| {
                                    state.volume = vol.clone();
                                    cx.notify();
                                });
                                cx.notify();
                            }
                            _ => {}
                        },
                    )
                    .detach();

                    cx.on_app_quit(|_, cx| {
                        let controller = cx.global::<Controller>().clone();

                        async move {
                            println!("exiting");
                            controller.exit();
                        }
                    })
                    .detach();
                    let titlebar = cx.new(|_| Titlebar::new());

                    let control_bar =
                        cx.new(|_| ControlBar::new(vol_slider.clone(), playbar.clone()));
                    let main_view = cx.new(|_| MainView::new());
                    let right_sidebar = cx.new(|cx| RightSidebar::new(cx));
                    let left_sidebar = cx.new(move |_| LeftSidebar::new(playlists.clone()));

                    Kagi {
                        titlebar,
                        res_handler,
                        left_sidebar,
                        control_bar,
                        main_view,
                        right_sidebar,
                    }
                })
            },
        )
        .unwrap();
    });
    Ok(())
}

pub fn lerp(start: Rgba, end: Rgba, t: f32) -> Rgba {
    Rgba {
        r: start.r + (end.r - start.r) * t,
        g: start.g + (end.g - start.g) * t,
        b: start.b + (end.b - start.b) * t,
        a: start.a + (end.a - start.a) * t,
    }
}
