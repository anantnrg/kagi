use std::{cell::Cell, rc::Rc};

use components::theme::Theme;
use gpui::*;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct MainView {
    pub now_playing: Entity<NowPlaying>,
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let np = self.now_playing.clone();
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .flex_col()
            .overflow_hidden()
            .gap_2()
            .child({
                if let Some(thumbnail) = np.read(cx).thumbnail.clone() {
                    div().w_24().h_24().child(
                        div()
                            .size_auto()
                            .flex()
                            .items_end()
                            .justify_end()
                            .bg(theme.secondary)
                            // .max_w(px(w.get()))
                            // .max_h(px(h.get()))
                            .child(img(thumbnail.img).size_full()),
                    )
                    // img(thumbnail.img).max_w_full().max_h_auto().debug()
                } else {
                    img("")
                }
            })
            .child({
                let np = np.read(cx);
                if !np.title.is_empty() {
                    div()
                        .text_color(theme.accent)
                        .child(np.title.clone())
                        .text_3xl()
                        .font_weight(FontWeight::BOLD)
                        .w_full()
                        .flex()
                        .flex_wrap()
                        .content_center()
                } else {
                    div().child("")
                }
            })
            .child({
                let np = np.read(cx);
                if !np.title.is_empty() {
                    div()
                        .text_color(theme.text)
                        .child(format!("{} • {}", np.artists.join(", ").clone(), np.album))
                        .text_xl()
                        .font_weight(FontWeight::MEDIUM)
                        .max_w(px(1280.0))
                        .flex()
                        .flex_wrap()
                        .content_center()
                } else {
                    div().text_color(theme.text).child("")
                }
            })
    }
}

impl MainView {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        MainView { now_playing }
    }
}
