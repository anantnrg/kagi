use gpui::{
    div, prelude::*, px, rgb, size, App, AppContext, Bounds, SharedString, TitlebarOptions, ViewContext, WindowBounds, WindowOptions
};

struct Reyvr {
    title: SharedString
}

impl Render for Reyvr {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        _cx.set_window_title(self.title.to_string().as_str());
        div()
            .flex()
            .bg(rgb(0x1e1e2d))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(format!("{}", &self.title))
    }
}

fn main() {
    App::new().run(|cx: &mut AppContext| {
            let bounds = Bounds::centered(None, size(px(500.), px(500.0)), cx);
            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    app_id: Some(String::from("reyvr")),
                    focus: true,
                    titlebar: Some(TitlebarOptions {
                        title: Some(SharedString::new_static("Reyvr")),
                        appears_transparent: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                },
                |cx| {
                    cx.new_view(|_cx| Reyvr {
                        title: "Reyvr - Nothing playing.".into(),
                    })
                },
            )
            .unwrap();
        });
    }
