use gpui::*;
use gpui_component::{
    button::*,
    slider::{Slider, SliderEvent, SliderState},
    *,
};

use crate::{
    audio::engine::PlaybackState,
    controller::player::{Controller, PlayerStateEvent},
};

pub struct Wiremann {
    pub vol_slider_state: Entity<SliderState>,
    pub playback_slider_state: Entity<SliderState>,
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

        let playback_slider_state = cx.new(|_| {
            SliderState::new()
                .min(0.0)
                .max(100.0)
                .default_value(0.0)
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

        cx.subscribe(
            &playback_slider_state,
            |_, _, event: &SliderEvent, cx| match event {
                SliderEvent::Change(value) => {
                    let controller = cx.global::<Controller>();
                    if controller.state.state == PlaybackState::Playing {
                        if let Some(meta) = controller.state.clone().meta {
                            controller.seek(slider_to_secs(value.start(), meta.duration));
                        }
                    }

                    cx.notify();
                }
            },
        )
        .detach();

        Self {
            vol_slider_state,
            playback_slider_state,
        }
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .v_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                div()
                    .gap_2()
                    .w_full()
                    .items_center()
                    .justify_center()
                    .child(
                        Button::new("load")
                            .primary()
                            .label("Load")
                            .on_click(|_, _, cx| {
                                cx.global::<Controller>().load(
                                    "E:\\music\\violence ft. doomguy\\454 - Bad and Boujee.mp3"
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
                    .child(div().w_24().child(Slider::new(&self.vol_slider_state)))
                    .child(div().w_48().child(Slider::new(&self.playback_slider_state))),
            )
    }
}

fn slider_to_secs(slider: f32, duration_secs: u64) -> u64 {
    ((slider.clamp(0.0, 100.0) / 100.0) * duration_secs as f32) as u64
}
