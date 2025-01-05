use anyhow::Error;
use components::button::Button;
use gpui::{
    App, AppContext, Bounds, SharedString, TitlebarOptions, ViewContext, WindowBounds,
    WindowOptions, div, prelude::*, px, rgb, size,
};
use std::sync::{Arc, Mutex};

struct Reyvr {
    title: SharedString,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.set_window_title(self.title.to_string().as_str());

        div()
            .flex()
            .gap_8()
            .bg(rgb(0x1e1e2d))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(Button::new().text("Play").on_click({
                //     let playbin = Arc::clone(&playbin);
                move |_, _| {
                    //         let playbin = playbin.lock().expect("Could not lock playbin");
                    //         playbin
                    //             .set_state(gstreamer::State::Playing)
                    //             .expect("Couldn't set playbin state to playing.");
                    println!("clicked");
                }
            }))
        // .child(Button::new().text("Pause").on_click({
        //     let playbin = Arc::clone(&playbin);
        //     move |_, _| {
        //         playbin
        //             .lock()
        //             .expect("Could not lock playbin")
        //             .set_state(gstreamer::State::Paused)
        //             .expect("Couldn't set playbin state to paused.");
        //     }
        // }))
        // .child(Button::new().text("+").size(40.0, 40.0).on_click({
        //     let playbin = Arc::clone(&playbin);
        //     let volume = Arc::clone(&volume);
        //     move |_, _| {
        //         let mut vol = volume.lock().expect("Could not lock volume");
        //         *vol += 0.2;
        //         if *vol > 1.0 {
        //             *vol = 1.0;
        //         }
        //         playbin
        //             .lock()
        //             .expect("Could not lock playbin")
        //             .set_property("volume", *vol);
        //         println!("volume set to: {}", *vol);
        //     }
        // }))
        // .child(Button::new().text("-").size(40.0, 40.0).on_click({
        //     let playbin = Arc::clone(&playbin);
        //     let volume = Arc::clone(&volume);
        //     move |_, _| {
        //         let mut vol = volume.lock().expect("Could not lock volume");
        //         *vol -= 0.2;
        //         if *vol < 0.0 {
        //             *vol = 0.0;
        //         }
        //         playbin
        //             .lock()
        //             .expect("Could not lock playbin")
        //             .set_property("volume", *vol);
        //         println!("volume set to: {}", *vol);
        //     }
        // }))
    }
}

fn main() -> Result<(), Error> {
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
    Ok(())
}
