use crate::controller::player::Controller;
use crate::ui::theme::Theme;

use crate::ui::icons::Icons;
use gpui::*;
use gpui_component::Icon;
use gpui_component::slider::{Slider, SliderState};

#[derive(Clone)]
pub struct ControlBar {
    pub playback_slider_state: Entity<SliderState>,
    pub vol_slider_state: Entity<SliderState>,
}

impl ControlBar {
    pub fn new(
        playback_slider_state: Entity<SliderState>,
        vol_slider_state: Entity<SliderState>,
    ) -> Self {
        ControlBar {
            playback_slider_state,
            vol_slider_state,
        }
    }
}

impl Render for ControlBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let state = cx.global::<Controller>().state.clone();

        div()
            .w_full()
            .h_32()
            .flex()
            .flex_col()
            .bg(theme.panel)
            .border_t_1()
            .border_color(theme.border)
            .child(
                div()
                    .w_full()
                    .h_1_2()
                    .flex()
                    .px_16()
                    .items_center()
                    .justify_center()
                    .child(format!("{}:{}", state.position / 60, state.position % 60))
                    .child(format!("{}:{}", state.duration / 60, state.duration % 60)),
            )
    }
}
