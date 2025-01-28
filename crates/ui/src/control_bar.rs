use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};
use gstreamer::State;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct ControlBar {
    now_playing: NowPlaying,
}

impl Render for ControlBar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>();
        div()
            .w_full()
            .h(px(72.0))
            .border_t_1()
            .bg(theme.background)
            .border_color(theme.secondary)
            .flex()
            .items_center()
            .justify_center()
            .child(
                div()
                    .size_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(svg().path("assets/play previous.svg").size_4())
                    .on_mouse_down(MouseButton::Left, {
                        {
                            let controller = controller.clone();
                            move |_, _, _| {
                                controller.prev();
                            }
                        }
                    }),
            )
            .child(
                div()
                    .size_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        svg()
                            .when(self.now_playing.state == State::Playing, |this| {
                                this.path("assets/pause.svg")
                            })
                            .when(self.now_playing.state == State::Paused, |this| {
                                this.path("assets/play.svg")
                            })
                            .size_4(),
                    )
                    .on_mouse_down(MouseButton::Left, {
                        {
                            let controller = controller.clone();
                            let np = self.now_playing.clone();
                            move |_, _, _| {
                                if np.state == State::Playing {
                                    controller.pause();
                                } else {
                                    controller.play();
                                }
                            }
                        }
                    }),
            )
            .child(
                div()
                    .size_5()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(svg().path("assets/play next.svg").size_4())
                    .on_mouse_down(MouseButton::Left, {
                        {
                            let controller = controller.clone();
                            move |_, _, _| {
                                controller.next();
                            }
                        }
                    }),
            )
    }
}

impl ControlBar {
    pub fn new(now_playing: NowPlaying) -> Self {
        ControlBar { now_playing }
    }
}
