use components::button::Button;
use gpui::*;
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Reyvr {
    pub title: SharedString,
    pub playbin: Arc<Mutex<gstreamer::Element>>,
    pub volume: Arc<Mutex<f32>>,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.set_window_title(self.title.to_string().as_str());
        let playbin = Arc::clone(&self.playbin);
        let volume = Arc::clone(&self.volume);

        div()
            .flex()
            .gap_8()
            .bg(rgb(0x1e1e2d))
            .size_full()
            .justify_center()
            .items_center()
            .text_color(rgb(0xffffff))
            .child(Button::new().text("Play").on_click({
                let playbin = Arc::clone(&playbin);
                move |_, _| {
                    let playbin = playbin.lock().expect("Could not lock playbin");
                    playbin
                        .set_state(gstreamer::State::Playing)
                        .expect("Couldn't set playbin state to playing.");
                }
            }))
            .child(Button::new().text("Pause").on_click({
                let playbin = Arc::clone(&playbin);
                move |_, _| {
                    playbin
                        .lock()
                        .expect("Could not lock playbin")
                        .set_state(gstreamer::State::Paused)
                        .expect("Couldn't set playbin state to paused.");
                }
            }))
            .child(Button::new().text("+").size(40.0, 40.0).on_click({
                let playbin = Arc::clone(&playbin);
                let volume = Arc::clone(&volume);
                move |_, _| {
                    let mut vol = volume.lock().expect("Could not lock volume");
                    *vol += 0.2;
                    if *vol > 1.0 {
                        *vol = 1.0;
                    }
                    playbin
                        .lock()
                        .expect("Could not lock playbin")
                        .set_property("volume", *vol);
                    println!("volume set to: {}", *vol);
                }
            }))
            .child(Button::new().text("-").size(40.0, 40.0).on_click({
                let playbin = Arc::clone(&playbin);
                let volume = Arc::clone(&volume);
                move |_, _| {
                    let mut vol = volume.lock().expect("Could not lock volume");
                    *vol -= 0.2;
                    if *vol < 0.0 {
                        *vol = 0.0;
                    }
                    playbin
                        .lock()
                        .expect("Could not lock playbin")
                        .set_property("volume", *vol);
                    println!("volume set to: {}", *vol);
                }
            }))
    }
}
