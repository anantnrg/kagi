use gpui::*;

#[derive(IntoElement)]
pub struct Titlebar;

impl RenderOnce for Titlebar {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div().w_full().h_8().bg(rgb(0xfff))
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
