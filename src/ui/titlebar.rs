use super::theme::Theme;

use gpui::*;
use gpui_component::{Icon, IconName, button::Button};

#[derive(Clone)]
pub struct Titlebar;

impl Render for Titlebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div().bg(theme.bg).size_full().child(
            div()
                .id("titlebar")
                .h_10()
                .w_full()
                .border_b_1()
                .border_color(theme.border)
                .bg(theme.panel)
                .window_control_area(WindowControlArea::Drag),
        )
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
