use components::theme::Theme;
use gpui::*;

use crate::{layout::Layout, player_context::PlayerContext};

#[derive(Clone)]
pub struct MainView;

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let meta = cx.global::<PlayerContext>().metadata.clone();
        let theme = cx.global::<Theme>();
        let layout = cx.global::<Layout>().clone();

        if !meta.read(cx).title.is_empty() {
            div()
                .track_focus(&cx.focus_handle())
                .bg(theme.main.bg)
                .w(px(layout.central_width.read(cx).clone()))
                .h_full()
                .flex()
                .rounded_lg()
                .flex_grow()
                .items_center()
                .justify_center()
                .flex_col()
                .overflow_hidden()
                .child({
                    if let Some(thumbnail) = meta.read(cx).thumbnail.clone() {
                        div()
                            .w(px(layout.central_width.read(cx).clone()))
                            .max_h(px(layout.central_width.read(cx).clone()))
                            .flex_col()
                            .flex()
                            .items_end()
                            .justify_end()
                            .flex_grow()
                            .child(
                                img(thumbnail.img)
                                    .size_full()
                                    .object_fit(ObjectFit::Contain),
                            )
                    } else {
                        div()
                    }
                })
                .child(div().h(px(16.0)))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .justify_start()
                        .items_center()
                        .w_full()
                        .flex_shrink_0()
                        .gap_2()
                        .child({
                            let meta = meta.read(cx);
                            if !meta.title.is_empty() {
                                div()
                                    .text_color(theme.main.title)
                                    .child(meta.title.clone())
                                    .text_3xl()
                                    .font_weight(FontWeight::EXTRA_BOLD)
                                    .w_full()
                                    .max_w_full()
                                    .text_align(TextAlign::Center)
                            } else {
                                div()
                            }
                        })
                        .child({
                            let meta = meta.read(cx);
                            if !meta.title.is_empty() {
                                div()
                                    .text_color(theme.main.artists)
                                    .text_xl()
                                    .font_weight(FontWeight::MEDIUM)
                                    .whitespace_normal()
                                    .child(format!("{} • {}", meta.artists.join(", "), meta.album))
                                    .w_full()
                                    .max_w_full()
                                    .text_align(TextAlign::Center)
                            } else {
                                div()
                            }
                        }),
                )
        } else {
            div()
                .track_focus(&cx.focus_handle())
                .bg(theme.main.bg)
                .w(px(layout.central_width.read(cx).clone()))
                .h_full()
                .flex()
                .rounded_lg()
                .flex_grow()
                .items_center()
                .justify_center()
                .flex_col()
                .overflow_hidden()
                .child(
                    div()
                        .w(px(layout.central_width.read(cx).clone()))
                        .max_h(px(layout.central_width.read(cx).clone()))
                        .flex_col()
                        .flex()
                        .items_end()
                        .justify_end()
                        .flex_grow(), // .child(
                                      //     img("logo/logo scaled.png")
                                      //         .size_full()
                                      //         .object_fit(ObjectFit::Contain),
                                      // ),
                )
                .child(
                    div()
                        .text_lg()
                        .text_color(theme.main.title)
                        .font_weight(FontWeight::BOLD)
                        .text_center()
                        .child("No playlist loaded..."),
                )
        }
    }
}

impl MainView {
    pub fn new() -> Self {
        MainView {}
    }
}
