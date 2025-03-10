use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

use crate::{
    layout::{Layout, LayoutMode},
    player_context::PlayerContext,
};

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
                .rounded_xl()
                .when(layout.mode.read(cx) == &LayoutMode::Overlay, |this| {
                    this.bg(theme.bg).rounded_none()
                })
                .flex_grow()
                .items_center()
                .justify_center()
                .flex_col()
                .overflow_hidden()
                .p_4()
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
                            .rounded_xl()
                            .child(
                                img(thumbnail.img)
                                    .size_full()
                                    .rounded_xl()
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
                        .gap_1()
                        .child({
                            let meta = meta.read(cx);
                            if !meta.title.is_empty() {
                                div()
                                    .text_color(theme.main.title)
                                    .child(meta.title.clone())
                                    .text_3xl()
                                    .when(layout.central_width.read(cx) < &600.0, |this| {
                                        this.text_2xl()
                                    })
                                    .when(layout.central_width.read(cx) < &400.0, |this| {
                                        this.text_xl()
                                    })
                                    .font_weight(FontWeight::BOLD)
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
                                    .when(layout.central_width.read(cx) < &600.0, |this| {
                                        this.text_lg()
                                    })
                                    .when(layout.central_width.read(cx) < &400.0, |this| {
                                        this.text_base()
                                    })
                                    .font_weight(FontWeight::NORMAL)
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
                        .text_xl()
                        .text_color(theme.main.title)
                        .font_weight(FontWeight::MEDIUM)
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
