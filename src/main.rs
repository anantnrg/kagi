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
        let playbin = Arc::clone(&self.playbin);
        div()
            .flex()
            .bg(rgb(0x1e1e2d))
            .size(px(500.0))
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(div()
                .flex()
                .w(px(150.))
                .h(px(30.))
                .bg(rgb(0x8266B7))
                .text_color(rgb(0x1e1e2d))
                .rounded_md()
                .justify_center()
                .content_center()
                .items_center()
                .child("Play")
                .on_mouse_down(MouseButton::Left, move |_, _| {
                    playbin
                        .lock()
                        .expect("Could not lock playbin")
                        .set_state(gstreamer::State::Playing)
                        .expect("Couldn't set playbin state to playing.");
                }))
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
