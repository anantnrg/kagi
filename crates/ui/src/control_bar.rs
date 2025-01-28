use backend::player::Controller;
use components::{
    icon::{Icon, Icons},
    theme::Theme,
};
use gpui::{prelude::FluentBuilder, *};
use gstreamer::State;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct ControlBar {
    now_playing: Entity<NowPlaying>,
}

impl Render for ControlBar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>();
        let np = self.now_playing.read(cx);
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
                    .child(
                        Icon::new(Icons::Previous)
                            .size(24.0)
                            .color(theme.text.into()),
                    )
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
                    .when(np.state == State::Null, |this| {
                        this.child(
                            Icon::new(Icons::Stopped)
                                .size(24.0)
                                .color(theme.text.into()),
                        )
                    })
                    .when(np.state == State::Playing, |this| {
                        this.child(Icon::new(Icons::Pause).size(24.0).color(theme.text.into()))
                    })
                    .when(np.state == State::Paused, |this| {
                        this.child(Icon::new(Icons::Play).size(24.0).color(theme.text.into()))
                    })
                    .on_mouse_down(MouseButton::Left, {
                        {
                            let controller = controller.clone();
                            let np = np.clone();
                            move |_, _, _| {
                                if np.state == State::Playing {
                                    controller.pause();
                                } else if np.state == State::Null || np.state == State::Paused {
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
                    .child(Icon::new(Icons::Next).size(24.0).color(theme.text.into()))
                    .on_mouse_down(MouseButton::Left, {
                        {
                            let controller = controller.clone();
                            move |_, _, cx| {
                                controller.next();
                            }
                        }
                    }),
            )
    }
}

impl ControlBar {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        ControlBar { now_playing }
    }
}
