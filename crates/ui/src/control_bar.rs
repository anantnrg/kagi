use backend::player::Controller;
use components::{
    icon::{Icon, Icons},
    slider::Slider,
    theme::Theme,
};
use gpui::{prelude::FluentBuilder, *};
use gstreamer::State;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct ControlBar {
    now_playing: Entity<NowPlaying>,
    vol_slider: Entity<Slider>,
}

impl Render for ControlBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let controller = cx.global::<Controller>();
        let theme = cx.global::<Theme>();

        let np = self.now_playing.read(cx);

        div()
            .w_full()
            .h(px(72.0))
            .border_t_1()
            .bg(theme.background)
            .border_color(theme.secondary)
            .flex()
            .justify_center()
            .justify_between()
            .px_4()
            .child(div().w_full().h_full())
            .child(
                div()
                    .flex()
                    .h_full()
                    .w_full()
                    .items_center()
                    .justify_center()
                    .overflow_hidden()
                    .gap_3()
                    .child(
                        div()
                            .size_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(
                                Icon::new(Icons::Previous)
                                    .size(32.0)
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
                            .size_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .when(np.state == State::Null, |this| {
                                this.child(
                                    Icon::new(Icons::Stopped)
                                        .size(32.0)
                                        .color(theme.text.into()),
                                )
                            })
                            .when(np.state == State::Playing, |this| {
                                this.child(
                                    Icon::new(Icons::Pause).size(32.0).color(theme.text.into()),
                                )
                            })
                            .when(np.state == State::Paused, |this| {
                                this.child(
                                    Icon::new(Icons::Play).size(32.0).color(theme.text.into()),
                                )
                            })
                            .on_mouse_down(MouseButton::Left, {
                                {
                                    let controller = controller.clone();
                                    let np = np.clone();
                                    move |_, _, _| {
                                        if np.state == State::Playing {
                                            controller.pause();
                                        } else if np.state == State::Null
                                            || np.state == State::Paused
                                        {
                                            controller.play();
                                        }
                                    }
                                }
                            }),
                    )
                    .child(
                        div()
                            .size_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(Icons::Next).size(32.0).color(theme.text.into()))
                            .on_mouse_down(MouseButton::Left, {
                                {
                                    let controller = controller.clone();
                                    move |_, _, _| {
                                        controller.next();
                                    }
                                }
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .h_auto()
                    .w_full()
                    .px_2()
                    .child(div().w_20().child(self.vol_slider.clone()))
                    .child(
                        div()
                            .child(format!("{:.0}%", np.volume * 100.0))
                            .text_color(theme.text)
                            .ml_4()
                            .w_10()
                            .overflow_hidden(),
                    ),
            )
    }
}

impl ControlBar {
    pub fn new(now_playing: Entity<NowPlaying>, vol_slider: Entity<Slider>) -> Self {
        ControlBar {
            now_playing,
            vol_slider,
        }
    }
}
