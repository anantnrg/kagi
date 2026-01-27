use crate::ui::theme::Theme;

use crate::ui::icons::Icons;
use gpui::*;
use gpui_component::Icon;

#[derive(Clone)]
pub struct ControlBar;

impl ControlBar {
    pub fn new() -> Self {
        ControlBar {}
    }
}

impl Render for ControlBar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h_32()
            .bg(theme.panel)
            .border_t_1()
            .border_color(theme.border)
    }
}
