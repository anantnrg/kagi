use backend::Backend;
use components::button::Button;
use gpui::*;
use std::sync::{Arc, Mutex};

pub struct Reyvr {
    pub title: SharedString,
    pub backend: Arc<dyn Backend>,
    pub volume: Arc<Mutex<f64>>,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        cx.set_window_title(self.title.to_string().as_str());
        let volume = Arc::clone(&self.volume);

        div()
            .flex()
            .gap_8()
            .bg(rgb(0x1e1e2d))
            .size_full()
            .justify_center()
            .items_center()
            .child(Button::new().text("Play").on_click({
                let backend = self.backend.clone();
                move |_, _| {
                    backend.play().expect("Could not play");
                }
            }))
            .child(Button::new().text("Pause").on_click({
                let backend = self.backend.clone();
                move |_, _| {
                    backend.pause().expect("Could not pause playback");
                }
            }))
            .child(Button::new().text("+").size(40.0, 40.0).on_click({
                let volume = Arc::clone(&volume);
                let backend = self.backend.clone();

                move |_, _| {
                    let mut vol = volume.lock().expect("Could not lock volume");
                    *vol += 0.2;
                    if *vol > 1.0 {
                        *vol = 1.0;
                    }
                    backend.set_volume(*vol).expect("Could not set volume");
                    println!("volume set to: {}", *vol);
                }
            }))
            .child(Button::new().text("-").size(40.0, 40.0).on_click({
                let backend = self.backend.clone();
                let volume = Arc::clone(&volume);
                move |_, _| {
                    let mut vol = volume.lock().expect("Could not lock volume");
                    *vol -= 0.2;
                    if *vol < 0.0 {
                        *vol = 0.0;
                    }
                    backend.set_volume(*vol).expect("Could not set volume");
                    println!("volume set to: {}", *vol);
                }
            }))
    }
}
