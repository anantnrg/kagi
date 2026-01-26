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
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();

        div()
            .h_full()
            .w_20()
            .flex()
            .flex_col()
            .flex_shrink_0()
            .items_center()
            .py_4()
            .gap_2()
            .border_r_1()
            .border_color(theme.border)
            .child(
                div()
                    .size_16()
                    .rounded_md()
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(IconName::Settings)),
            )
    }
}
