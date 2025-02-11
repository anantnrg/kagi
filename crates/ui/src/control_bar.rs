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
    playbar: Entity<Slider>,
}

impl Render for ControlBar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let controller = cx.global::<Controller>();
        let theme = cx.global::<Theme>();

        let np = self.now_playing.read(cx);
        let np_write = self.now_playing.clone();

        div()
            .w_full()
            .h_24()
            .border_t_1()
            .bg(theme.background)
            .border_color(theme.secondary)
            .flex()
            .flex_col()
            .justify_center()
            .justify_between()
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
                    .text_color(theme.text)
                    .font_weight(FontWeight::MEDIUM)
                    .child(format!("{:02}:{:02}", np.position / 60, np.position % 60))
                    .child(
                        div()
                            .mx_3()
                            .w_full()
                            .h_6()
                            .pt(px(10.0))
                            .child(self.playbar.clone()),
                    )
                    .child(format!("{:02}:{:02}", np.duration / 60, np.duration % 60)),
            )
            .child(
                div()
                    .w_full()
                    .h_full()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .w_full()
                            .h_full()
                            .when(win.bounds().size.width.0 < 400.0, |this| {
                                this.w_auto().flex_grow()
                            }),
                    )
                    .child(
                        div()
                            .flex()
                            .h_full()
                            .w_full()
                            .items_center()
                            .justify_center()
                            .overflow_hidden()
                            .gap_x_4()
                            .child(
                                div()
                                    .size_6()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Shuffle)
                                            .size(24.0)
                                            .color(theme.text)
                                            .hover(theme.accent)
                                            .when(np.shuffle, |this| this.color(theme.accent)),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            let controller = controller.clone();
                                            move |_, _, _| {
                                                controller.shuffle();
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
                                            .size(24.0)
                                            .color(theme.text)
                                            .hover(theme.accent),
                                    )
                                    .hover(|this| this.text_color(theme.accent))
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
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .when(np.state == State::Null, |this| {
                                        this.child(
                                            Icon::new(Icons::Stopped)
                                                .size(24.0)
                                                .color(theme.text)
                                                .hover(theme.accent),
                                        )
                                    })
                                    .when(np.state == State::Playing, |this| {
                                        this.child(
                                            Icon::new(Icons::Pause)
                                                .size(24.0)
                                                .color(theme.text)
                                                .hover(theme.accent),
                                        )
                                    })
                                    .when(np.state == State::Paused, |this| {
                                        this.child(
                                            Icon::new(Icons::Play)
                                                .size(24.0)
                                                .color(theme.text)
                                                .hover(theme.accent),
                                        )
                                    })
                                    .hover(|this| this.text_color(theme.accent))
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
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        Icon::new(Icons::Next)
                                            .size(24.0)
                                            .color(theme.text)
                                            .hover(theme.accent),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            let controller = controller.clone();
                                            move |_, _, _| {
                                                controller.next();
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
                                            .size(24.0)
                                            .color(theme.text)
                                            .hover(theme.accent)
                                            .when(np.repeat, |this| this.color(theme.accent)),
                                    )
                                    .on_mouse_down(MouseButton::Left, {
                                        {
                                            move |_, _, cx| {
                                                np_write.update(cx, |this, _| {
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
                            .child(div().w_20().child(self.vol_slider.clone()))
                            .child(
                                div()
                                    .child(format!("{:.0}%", np.volume * 100.0))
                                    .text_color(theme.text)
                                    .ml_4()
                                    .w_10()
                                    .overflow_hidden(),
                            ),
                    ),
            )
    }
}

impl ControlBar {
    pub fn new(
        now_playing: Entity<NowPlaying>,
        vol_slider: Entity<Slider>,
        playbar: Entity<Slider>,
    ) -> Self {
        ControlBar {
            now_playing,
            vol_slider,
            playbar,
        }
    }
}
