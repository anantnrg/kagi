use super::{now_playing::*, titlebar::Titlebar};
use crate::layout::Layout;
use backend::player::Controller;
use components::{button::Button, slider::Slider, theme::Theme};
use gpui::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Reyvr {
    pub vol_slider: View<Slider>,
    pub layout: Layout,
    pub now_playing: Model<NowPlaying>,
    pub theme: Theme,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let titlebar = cx.new_view(|_| Titlebar::new(self.now_playing.clone(), theme.clone()));

        div()
            .w_full()
            .h_full()
            .flex_col()
            .child(titlebar.clone())
            .child(
                div()
                    .flex()
                    .gap_8()
                    .bg(theme.background)
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child(Button::new().text("Play").on_click({
                        let now_playing = self.now_playing.clone();
                        let controller = cx.global::<Controller>();
                        move |_, cx| {}
                    }))
                    .child(Button::new().text("Pause").on_click({
                        let app = self.clone();
                        let playlist = app.playlist.clone();

                        move |_, _| {
                            if playlist.lock().expect("Could not lock playlist").playing == true {
                                // app.backend.pause().expect("Could not pause playback");
                            }
                        }
                    }))
                    .child(Button::new().text("Load Playlist").on_click({
                        let app = self.clone();
                        let playlist = app.playlist.clone();

                        move |_, _| {
                            // let new_playlist = Playlist::from_dir(
                            //     &app.backend.clone(),
                            //     PathBuf::from("E:\\music\\PSYCHX - Kordhell, Scarlxrd"),
                            // );
                            // *playlist.lock().expect("Could not lock playlist") = new_playlist;
                            println!("Playlist loaded");
                        }
                    }))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_auto()
                            .w_20()
                            .child(self.vol_slider.clone()),
                    )
                    .child(div().w_10().child(format!("{}", self.volume.clone()))),
            )
    }
}
