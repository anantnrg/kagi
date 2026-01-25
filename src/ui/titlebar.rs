use super::theme::Theme;

use gpui::*;
use gpui_component::{Icon, IconName, button::Button};

#[derive(Clone)]
pub struct Titlebar;

impl Render for Titlebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .id("titlebar")
            .h_10()
            .w_full()
            .flex()
            .items_center()
            .justify_between()
            .border_b_1()
            .border_color(theme.border)
            .bg(theme.panel)
            .child(div().flex().flex_1().h_full().w_auto().flex())
            .child(
                div()
                    .flex()
                    .h_full()
                    .flex_1()
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                div()
                    .h_full()
                    .w_auto()
                    .flex()
                    .flex_1()
                    .items_center()
                    .justify_center()
                    .child("Wiremann")
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                div()
                    .flex()
                    .h_full()
                    .flex_1()
                    .window_control_area(WindowControlArea::Drag),
            )
            .child(
                div()
                    .h_full()
                    .flex_1()
                    .flex()
                    .justify_end()
                    .child(
                        div()
                            .id("win_min")
                            .h_full()
                            .w_12()
                            .flex()
                            .items_center()
                            .justify_center()
                            .child(Icon::new(IconName::WindowMinimize))
                            .hover(|this| this.bg(theme.highlighted))
                            .window_control_area(WindowControlArea::Min),
                    )
                    .child(
                        div()
                            .id("win_max")
                            .h_full()
                            .w_12()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(theme.highlighted))
                            .child(if win.is_maximized() {
                                Icon::new(IconName::WindowRestore)
                            } else {
                                Icon::new(IconName::WindowMaximize)
                            })
                            .window_control_area(WindowControlArea::Max),
                    )
                    .child(
                        div()
                            .h_full()
                            .w_12()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0xe81123)))
                            .child(Icon::new(IconName::WindowClose))
                            .window_control_area(WindowControlArea::Close),
                    ),
            )
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
