use super::Page;
use crate::ui::theme::Theme;

use gpui::*;
use gpui_component::{Icon, IconName};

#[derive(Clone)]
pub struct NavBar {
    page: Page,
}

impl NavBar {
    pub fn new() -> Self {
        NavBar { page: Page::Home }
    }
}

impl Render for NavBar {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
    }
}
