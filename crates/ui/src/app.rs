use super::{now_playing::*, titlebar::Titlebar};
use crate::layout::Layout;
use backend::player::Controller;
use components::{button::Button, slider::Slider, theme::Theme};
use gpui::*;

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
                        // let now_playing = self.now_playing.clone();
                        move |_, cx| {
                            cx.global::<Controller>().play();
                        }
                    }))
                    .child(Button::new().text("Pause").on_click({
                        move |_, cx| {
                            cx.global::<Controller>().pause();
                        }
                    }))
                    .child(Button::new().text("Load Playlist").on_click({
                        move |_, cx| {
                            cx.global::<Controller>()
                                .load("E:\\music\\PSYCHX - Kordhell, Scarlxrd");
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
                    ), // .child(div().w_10().child(format!("{}", self.volume.clone()))),
            )
    }
}
