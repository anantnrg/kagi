use super::{theme::Theme, titlebar::Titlebar};
use crate::{audio::engine::PlaybackState, controller::player::Controller};
use gpui::*;
use gpui_component::{
    button::*,
    slider::{Slider, SliderEvent, SliderState},
};

pub struct Wiremann {
    pub vol_slider_state: Entity<SliderState>,
    pub playback_slider_state: Entity<SliderState>,
    pub titlebar: Entity<Titlebar>,
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

        cx.set_global(Theme::default());

        let titlebar = cx.new(|_| Titlebar::new());

        Self {
            vol_slider_state,
            playback_slider_state,
            titlebar,
        }
    }
}

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().bg(theme.bg).size_full().child(
            div()
                .size_10()
                .bg(theme.panel)
                .hover(|this| this.bg(theme.accent)),
        )
    }
}

fn slider_to_secs(slider: f32, duration_secs: u64) -> u64 {
    ((slider.clamp(0.0, 100.0) / 100.0) * duration_secs as f32) as u64
}
