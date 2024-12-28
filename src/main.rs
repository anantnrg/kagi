use anyhow::Error;
use gpui::{
    div, prelude::*, px, rgb, size, App, AppContext, Bounds, MouseButton, SharedString, TitlebarOptions, ViewContext, WindowBounds, WindowOptions
};
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};

struct Reyvr {
    title: SharedString,
    playbin: Arc<Mutex<gstreamer::Element>>,
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
            .child(div().child("Play").on_mouse_down(MouseButton::Left, |_, _| {println!("clicked")}))
    }
}

fn main() -> Result<(), Error> {
    gstreamer::init()?;
    
    let uri = "file:///D:/repos/reyvr/assets/music.mp3";
    let playbin = Arc::new(Mutex::new(
            gstreamer::ElementFactory::make("playbin")
                .name("playbin")
                .property("uri", uri)
                .build()
                .expect("Could not initialize playbin."),
        ));
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
                    playbin
                })
            },
        )
        .unwrap();
    });
    Ok(())
}
