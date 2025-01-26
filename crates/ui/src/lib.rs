pub mod app;
pub mod assets;
pub mod layout;
pub mod now_playing;
pub mod res_handler;
pub mod titlebar;

use app::Reyvr;
use assets::*;
use backend::{
    Backend,
    playback::Playlist,
    player::{Controller, Player, Response},
};
use components::{
    slider::{Slider, SliderEvent},
    theme::Theme,
};
use gpui::*;
use layout::Layout;
use now_playing::{NowPlaying, NowPlayingEvent};
use res_handler::ResHandler;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let app = App::new().with_assets(Assets {
        base: PathBuf::from("assets"),
    });

    app.run(move |cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.0), px(500.0)), cx);

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
            |cx| {
                cx.new_view(|cx| {
                    let theme = Theme::default();
                    let now_playing = NowPlaying::new();
                    let np = cx.new_model(|_| now_playing.clone());
                    let res_handler = cx.new_model(|_| ResHandler {});
                    let arc_res = Arc::new(res_handler.clone());
                    let (mut player, controller) =
                        Player::new(backend.clone(), Arc::new(Mutex::new(Playlist::default())));
                    let vol_slider = cx.new_view(|_| {
                        Slider::new(theme)
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.4)
                    });
                    let recv_controller = controller.clone();

                    cx.set_global(controller);
                    cx.background_executor()
                        .spawn(async move {
                            player.run().await;
                        })
                        .detach();
                    cx.spawn(|_, cx| async move {
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
                    cx.subscribe(&vol_slider, |_, _, event: &SliderEvent, cx| match event {
                        SliderEvent::Change(vol) => {
                            let volume = (vol * 100.0).round() as f64 / 100.0;
                            cx.global::<Controller>().volume(volume);

                            cx.notify();
                        }
                    })
                    .detach();
                    cx.subscribe(
                        &np,
                        |this: &mut Reyvr, _, event: &NowPlayingEvent, cx| match event {
                            NowPlayingEvent::Meta(title, album, artists, duration) => {
                                this.now_playing.update(cx, |this, _| {
                                    this.title = title.clone();
                                    this.album = album.clone();
                                    this.artists = artists.clone();
                                    this.duration = duration.clone();
                                });
                                cx.notify();
                            }
                            NowPlayingEvent::Position(pos) => {
                                this.now_playing.update(cx, |this, _| {
                                    this.position = *pos;
                                });
                                cx.notify();
                            }
                            NowPlayingEvent::Thumbnail(img) => {
                                this.now_playing.update(cx, |this, _| {
                                    this.thumbnail = Some(img.clone());
                                });
                                cx.notify();
                            }
                        },
                    )
                    .detach();
                    cx.subscribe(
                        &res_handler,
                        move |this: &mut Reyvr, _, event: &Response, cx| match event {
                            Response::Eos => {
                                println!("End of stream");
                                cx.global::<Controller>().next();
                            }
                            Response::Position(pos) => this.now_playing.update(cx, |np, cx| {
                                np.update_pos(cx, *pos);
                            }),
                            Response::StreamStart => cx.global::<Controller>().get_meta(),
                            Response::Metadata(track) => {
                                this.now_playing.update(cx, |np, cx| {
                                    let track = track.clone();
                                    np.update_meta(
                                        cx,
                                        track.title.into(),
                                        track.album.into(),
                                        track.artists.iter().map(|s| s.clone().into()).collect(),
                                        track.duration,
                                    );
                                });
                            }
                            Response::Thumbnail(art) => {
                                this.now_playing.update(cx, |np, cx| {
                                    np.update_thumbnail(
                                        cx,
                                        ImageSource::Render(RenderImage::new(art.clone()).into()),
                                    );
                                });
                            }
                            _ => {}
                        },
                    )
                    .detach();
                    Reyvr {
                        layout: Layout::new(),
                        now_playing: np,
                        theme,
                        vol_slider,
                        res_handler,
                    }
                })
            },
        )
        .unwrap();
    });
    Ok(())
}
