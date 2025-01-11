use crate::now_playing::NowPlaying;

use components::icon::*;
use gpui::*;

pub struct Titlebar {
    now_playing: Model<NowPlaying>,
}

impl Render for Titlebar {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .w_full()
            .h_9()
            .bg(rgb(0x1e1d2d))
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .w_9()
                    .h_full()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(Icons::LeftSidebar).size(20.0).color(0xcdd6f4)),
            )
            .child(format!(
                "Reyvr - Playing {}",
                self.now_playing.read(cx).title
            ))
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
