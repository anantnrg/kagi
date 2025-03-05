use backend::player::Controller;
use components::{
    icon::{Icon, Icons},
    slider::Slider,
    theme::Theme,
};
use gpui::{prelude::FluentBuilder, *};
use gstreamer::State;

use crate::player_context::PlayerContext;

#[derive(Clone)]
pub struct ControlBar {
    vol_slider: Entity<Slider>,
    playbar: Entity<Slider>,
}

impl Render for ControlBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let state_write = cx.global_mut::<PlayerContext>().state.clone();
        let theme = cx.global::<Theme>();

        let meta = cx.global::<PlayerContext>().metadata.read(cx);
        let state = cx.global::<PlayerContext>().state.read(cx);

        div()
            .track_focus(&cx.focus_handle())
            .w_full()
            .h_24()
            .border_t_1()
            .bg(theme.control_bar.bg)
            .flex()
            .flex_col()
            .justify_center()
            .justify_between()
            .occlude()
            .px_4()
            .child(
                div()
                    .w_full()
                    .id("playbar")
                    .overflow_hidden()
                    .h_auto()
                    .flex()
                    .items_center()
                    .justify_center()
                    .gap_x_2()
                    .px_3()
                    .mt_3()
                    .pb_1()
                    .text_sm()
                    .text_color(theme.control_bar.text)
                    .font_weight(FontWeight::MEDIUM)
                    .child(format!(
                        "{:02}:{:02}",
                        state.position / 60,
                        state.position % 60
                    ))
                    .child(
                        div()
                            .mx_3()
                            .w_full()
                            .h_6()
                            .pt(px(10.0))
                            .child(self.playbar.clone()),
                    )
                    .child(format!(
                        "{:02}:{:02}",
                        meta.duration / 60,
                        meta.duration % 60
                    )),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(div().w_full().h_full())
                    .child(
                        div()
                            .flex()
                            .h_full()
                            .w_auto()
                            .items_center()
                            .justify_center()
                            .overflow_hidden()
                            .gap_x_4()
                            .flex_shrink_0()
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Shuffle)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover)
                                            .when(state.shuffle, |this| {
                                                this.color(theme.control_bar.hover)
                                            }),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                cx.global::<Controller>().shuffle();
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Previous)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover),
                                    )
                                    .hover(|this| this.text_color(theme.control_bar.hover))
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                cx.global::<Controller>().prev();
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Rewind)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                let current_pos = cx
                                                    .global::<PlayerContext>()
                                                    .state
                                                    .read(cx)
                                                    .position;
                                                cx.global::<Controller>()
                                                    .seek(current_pos.saturating_sub(5));
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .when(state.state == State::Null, |this| {
                                        this.child(
                                            Icon::new(Icons::Stopped)
                                                .size(22.0)
                                                .color(theme.control_bar.text)
                                                .hover(theme.control_bar.hover),
                                        )
                                    })
                                    .when(state.state == State::Playing, |this| {
                                        this.child(
                                            Icon::new(Icons::Pause)
                                                .size(22.0)
                                                .color(theme.control_bar.text)
                                                .hover(theme.control_bar.hover),
                                        )
                                    })
                                    .when(state.state == State::Paused, |this| {
                                        this.child(
                                            Icon::new(Icons::Play)
                                                .size(22.0)
                                                .color(theme.control_bar.text)
                                                .hover(theme.control_bar.hover),
                                        )
                                    })
                                    .hover(|this| this.text_color(theme.control_bar.hover))
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            let np = state.clone();
                                            move |_, _, cx| {
                                                if np.state == State::Playing {
                                                    cx.global::<Controller>().pause();
                                                } else if np.state == State::Null
                                                    || np.state == State::Paused
                                                {
                                                    cx.global::<Controller>().play();
                                                }
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::FastForward)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                let current_pos = cx
                                                    .global::<PlayerContext>()
                                                    .state
                                                    .read(cx)
                                                    .position;
                                                let total_duration = cx
                                                    .global::<PlayerContext>()
                                                    .metadata
                                                    .read(cx)
                                                    .duration;
                                                cx.global::<Controller>().seek(
                                                    (current_pos + 5).clamp(0, total_duration),
                                                );
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Next)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                cx.global::<Controller>().next();
                                            }
                                        }
                                    }),
                            )
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Repeat)
                                            .size(22.0)
                                            .color(theme.control_bar.text)
                                            .hover(theme.control_bar.hover)
                                            .when(state.repeat, |this| {
                                                this.color(theme.control_bar.hover)
                                            }),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                state_write.update(cx, |this, _| {
                                                    this.repeat = !this.repeat
                                                })
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
                            .gap_x_2()
                            .child({
                                let vol = cx.global::<PlayerContext>().state.read(cx).volume as f32;
                                if vol == 0.0 {
                                    Icon::new(Icons::VolZero)
                                        .size(22.0)
                                        .color(theme.control_bar.text)
                                        .hover(theme.control_bar.hover)
                                } else if vol <= 0.3 {
                                    Icon::new(Icons::VolLow)
                                        .size(22.0)
                                        .color(theme.control_bar.text)
                                        .hover(theme.control_bar.hover)
                                } else if vol <= 0.7 {
                                    Icon::new(Icons::VolMid)
                                        .size(22.0)
                                        .color(theme.control_bar.text)
                                        .hover(theme.control_bar.hover)
                                } else {
                                    Icon::new(Icons::VolHigh)
                                        .size(22.0)
                                        .color(theme.control_bar.text)
                                        .hover(theme.control_bar.hover)
                                }
                            })
                            .child(
                                div()
                                    .flex()
                                    .h_full()
                                    .items_center()
                                    .child(div().w_20().child(self.vol_slider.clone()))
                                    .child(
                                        div()
                                            .child(format!("{:00.0}%", state.volume * 100.0))
                                            .text_sm()
                                            .text_color(theme.control_bar.text)
                                            .ml_4()
                                            .w_10()
                                            .overflow_hidden(),
                                    ),
                            ),
                    ),
            )
    }
}

impl ControlBar {
    pub fn new(vol_slider: Entity<Slider>, playbar: Entity<Slider>) -> Self {
        ControlBar {
            vol_slider,
            playbar,
        }
    }
}
