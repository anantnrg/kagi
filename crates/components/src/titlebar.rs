use super::icon::*;
use gpui::*;

#[derive(IntoElement)]
pub struct Titlebar;

impl RenderOnce for Titlebar {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .w_full()
            .h_8()
            .bg(rgb(0x1e1d2d))
            .flex()
            .items_center()
            .justify_between()
            .child("close")
            .child("Reyvr - Playing nothing")
            .child(
                div()
                    .flex()
                    .items_center()
                    .w_auto()
                    .h_full()
                    .child(Icon::new(Icons::Minimize))
                    .child(Icon::new(Icons::Maximize))
                    .child(Icon::new(Icons::Close)),
            )
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
