pub mod app;
pub mod assets;
pub mod control_bar;
mod keybinds;
pub mod layout;
pub mod main_view;
pub mod player_context;
pub mod queue_list;
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
use layout::Layout;
use main_view::MainView;
use player_context::{PlayerContext, PlayerStateEvent, Thumbnail, Track};
use queue_list::QueueList;
use res_handler::ResHandler;
use sidebar::LeftSidebar;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use titlebar::Titlebar;

actions!(text_input, [
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
]);

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let app = Application::new().with_assets(Assets {
        base: PathBuf::from("assets"),
    });

    app.run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);
        components::input::bind_actions(cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                app_id: Some(String::from("reyvr")),
                focus: true,
                titlebar: Some(TitlebarOptions {
                    title: None,
                    appears_transparent: true,
                    ..Default::default()
                }),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| {
                    let theme = Theme::default();
                    let player_context = PlayerContext::new(cx);
                    let res_handler = cx.new(|_| ResHandler {});
                    let arc_res = Arc::new(res_handler.clone());
                    let (mut player, controller) =
                        Player::new(backend.clone(), Arc::new(Mutex::new(Playlist::default())));
                    let vol_slider = cx.new(|_| {
                        Slider::new(theme)
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.2)
                    });
                    let playbar = cx.new(|_| {
                        Slider::new(theme)
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.0)
                    });
                    let layout = Layout::new(cx);
                    let recv_controller = controller.clone();
                    let saved_playlists = cx.new(|_| SavedPlaylists::default());
                    let playlists = saved_playlists.clone();

                    keybinds::register(cx);
                    cx.set_global(controller);
                    cx.set_global(theme);
                    cx.set_global(player_context.clone());
                    cx.set_global(layout);
                    cx.background_executor()
                        .spawn(async move {
                            player.run().await;
                        })
                        .detach();
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
                                metadata.update(cx, |meta, cx| {
                                    let track = track.clone();
                                    meta.title = track.title.into();
                                    meta.album = track.album.into();
                                    meta.artists =
                                        track.artists.iter().map(|s| s.clone().into()).collect();
                                    meta.duration = track.duration;
                                    cx.notify();
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
                                })
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
                            _ => {}
                        },
                    )
                    .detach();

                    let titlebar = cx.new(|_| Titlebar::new());

                    let control_bar =
                        cx.new(|_| ControlBar::new(vol_slider.clone(), playbar.clone()));
                    let main_view = cx.new(|_| MainView::new());
                    let queue_list = cx.new(|cx| QueueList::new(cx));
                    let left_sidebar = cx.new(move |_| LeftSidebar::new(playlists.clone()));
                    cx.global::<Controller>().load_saved_playlists();

                    Kagi {
                        titlebar,
                        res_handler,
                        left_sidebar,
                        control_bar,
                        main_view,
                        queue_list,
                    }
                })
            },
        )
        .unwrap();
    });
    Ok(())
}
