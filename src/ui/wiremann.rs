use gpui::*;
use gpui_component::{
    button::*,
    slider::{Slider, SliderEvent, SliderState},
    *,
};

use crate::controller::player::{Controller, PlayerState};

pub struct Wiremann {
    pub vol_slider_state: Entity<SliderState>,
}

impl Wiremann {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let vol_slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(100.0)
                .step(1.0)
        });

        cx.subscribe(
            &vol_slider_state,
            |_, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(value) => {
                    cx.global::<Controller>().volume(value.start());
                    cx.notify();
                }
            },
        )
        .detach();

        Self { vol_slider_state }
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                Button::new("load")
                    .primary()
                    .label("Load")
                    .on_click(|_, _, cx| {
                        cx.global::<Controller>().load(
                            "E:\\music\\violence ft. doomguy\\463 - NF - PAID MY DUES.mp3"
                                .to_string(),
                        )
                    }),
            )
            .child(
                Button::new("play")
                    .primary()
                    .label("Play")
                    .on_click(|_, _, cx| cx.global::<Controller>().play()),
            )
            .child(
                Button::new("pause")
                    .primary()
                    .label("Pause")
                    .on_click(|_, _, cx| cx.global::<Controller>().pause()),
            )
            .child(text::Text::String(SharedString::from(
                cx.global::<Controller>().state.position.to_string(),
            )))
            .child(Slider::new(&self.vol_slider_state))
    }
}
