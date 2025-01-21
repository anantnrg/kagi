pub mod app;
pub mod assets;
pub mod layout;
pub mod now_playing;
pub mod titlebar;

use app::Reyvr;
use assets::*;
use backend::{Backend, playback::Playlist};
use components::{
    slider::{Slider, SliderEvent},
    theme::Theme,
};
use gpui::*;
use layout::Layout;
use now_playing::{NowPlaying, NowPlayingEvent};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let app = App::new().with_assets(Assets {
        base: PathBuf::from("assets"),
    });

    app.run(|cx: &mut AppContext| {
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
                    let vol_slider = cx.new_view(|_| {
                        Slider::new(theme)
                            .min(0.0)
                            .max(1.0)
                            .step(0.005)
                            .default(0.4)
                    });
                    cx.subscribe(
                        &vol_slider,
                        |this: &mut Reyvr, _, event: &SliderEvent, cx| match event {
                            SliderEvent::Change(vol) => {
                                this.volume = (vol * 100.0).round() as f64 / 100.0;
                                let app = this.clone();
                                let backend = app.backend.clone();
                                let playlist = app.playlist.clone();
                                if playlist.lock().expect("Could not lock playlist").playing == true
                                {
                                    // backend
                                    //     .set_volume(*vol as f64)
                                    //     .expect("Could not set volume");
                                    println!("volume set to: {}", *vol);
                                }
                                cx.notify();
                            }
                        },
                    )
                    .detach();
                    cx.subscribe(&np, |this, _, event: &NowPlayingEvent, cx| match event {
                        NowPlayingEvent::Update(title, album, artists) => {
                            this.now_playing.update(cx, |this, _| {
                                this.title = title.clone();
                                this.album = album.clone();
                                this.artists = artists.clone();
                            });
                            cx.notify();
                        }
                    })
                    .detach();
                    Reyvr {
                        backend,
                        playlist: Arc::new(Mutex::new(Playlist::default())),
                        volume: 0.5,
                        layout: Layout::new(),
                        now_playing: np,
                        theme,
                        vol_slider,
                    }
                })
            },
        )
        .unwrap();
    });
    Ok(())
}
