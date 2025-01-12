use crate::now_playing::NowPlaying;

use components::icon::*;
use gpui::*;
use prelude::FluentBuilder;

pub struct Titlebar {
    now_playing: Model<NowPlaying>,
}

impl Render for Titlebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_8()
            .bg(rgb(0x1e1d2d))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .w_6()
                    .h_6()
                    .ml(px(6.0))
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .hover(|this| this.bg(rgb(0x45475a)))
                    .child(Icon::new(Icons::LeftSidebar).size(18.0).color(0xcdd6f4)),
            )
            .child(
                div()
                    .flex()
                    .w_auto()
                    .h_full()
                    .items_center()
                    .child(div().child("Reyvr").text_color(rgb(0xca9ee6)))
                    .child({
                        let np = self.now_playing.read(cx);
                        div().when(np.title.len() != 0, |this| {
                            this.child(format!(" - {}", np.title))
                                .truncate()
                                .text_ellipsis()
                                .overflow_hidden()
                                .whitespace_nowrap()
                                .text_color(rgb(0xcdd6f4))
                                .text_sm()
                        })
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .w_auto()
                    .h_full()
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .child(Icon::new(Icons::Minimize).size(20.0).color(0xffffff)),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .child({
                                if cx.is_maximized() {
                                    Icon::new(Icons::Restore).size(20.0).color(0xffffff)
                                } else {
                                    Icon::new(Icons::Maximize).size(20.0).color(0xffffff)
                                }
                            }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0xe81123)))
                            .child(Icon::new(Icons::Close).size(20.0).color(0xffffff)),
                    ),
            )
    }
}

impl Titlebar {
    pub fn new(now_playing: Model<NowPlaying>) -> Titlebar {
        Titlebar { now_playing }
    }
}
