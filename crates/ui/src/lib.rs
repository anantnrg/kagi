pub mod app;
pub mod assets;
pub mod layout;

use app::Reyvr;
use assets::*;
use backend::Backend;
use gpui::*;
use layout::Layout;
use std::sync::{Arc, Mutex};

pub fn run_app(backend: Arc<dyn Backend>) -> anyhow::Result<()> {
    let app = App::new();

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
                cx.new_view(|_cx| Reyvr {
                    title: "Reyvr - Nothing playing.".into(),
                    backend,
                    volume: Arc::new(Mutex::new(0.5)),
                    layout: Layout::new(),
                })
            },
        )
        .unwrap();
    });
    Ok(())
}
