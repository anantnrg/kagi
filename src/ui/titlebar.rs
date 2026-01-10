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
                .size_10()
                .bg(theme.panel)
                .hover(|this| this.bg(theme.accent)),
        )
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
