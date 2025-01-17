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
use now_playing::NowPlaying;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let app = App::new().with_assets(Assets {
        base: PathBuf::from("assets"),
    });

    app.run(|cx: &mut AppContext| {
        let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
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
                    let vol_slider = cx
                        .new_view(|_| Slider::new(theme).min(0.0).max(1.0).step(0.1).default(0.4));
                    cx.subscribe(
                        &vol_slider,
                        |this: &mut Reyvr, _, event: &SliderEvent, cx| match event {
                            SliderEvent::Change(vol) => {
                                this.volume = Arc::new(Mutex::new(*vol as f64));
                                cx.notify();
                            }
                        },
                    )
                    .detach();
                    Reyvr {
                        backend,
                        playlist: Arc::new(Mutex::new(Playlist::default())),
                        volume: Arc::new(Mutex::new(0.5)),
                        layout: Layout::new(),
                        now_playing: NowPlaying::new(),
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
