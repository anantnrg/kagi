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
            .child({
                if let Some(thumbnail) = np.read(cx).thumbnail.clone() {
                        div()
                            .size_auto()
                            .flex()
                            .items_end()
                            .justify_end()
                            .bg(theme.secondary)
                            // .max_w(px(w.get()))
                            // .max_h(px(h.get()))
                            .child(img(thumbnail.img).size_full()),
                    // img(thumbnail.img)
                    //     .max_w_full()
                    //     .h_full()
                    //     .rounded_lg()
                    //     .flex()
                    //     .justify_end()
                    //     .items_center()
                } else {
                    img("")
                }
            })
            .child(
                div()
                    .flex()
                    .flex_col()
                    .justify_center()
                    .items_center()
                    .w_full()
                    .h_full()
                    .gap_2()
                    .child({
                        let np = np.read(cx);
                        if !np.title.is_empty() {
                            div()
                                .text_color(theme.accent)
                                .child(div().child(np.title.clone()))
                                .text_3xl()
                                .font_weight(FontWeight::EXTRA_BOLD)
                                .id("id")
                                .w_full()
                                .text_align(TextAlign::Center)
                                .max_w_full()
                        } else {
                            div().id("")
                        }
                    })
                    .child({
                        let np = np.read(cx);
                        if !np.title.is_empty() {
                            div()
                                .text_color(theme.text)
                                .text_xl()
                                .font_weight(FontWeight::MEDIUM)
                                .whitespace_normal()
                                .child(format!("{} • {}", np.artists.join(", ").clone(), np.album))
                                .id("id")
                                .w_full()
                                .text_align(TextAlign::Center)
                                .max_w_full()
                        } else {
                            div().text_color(theme.text).id("")
                        }
                    }),
            )
    }
}

impl MainView {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        MainView { now_playing }
    }
}
