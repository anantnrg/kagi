use gpui::*;

#[derive(Clone)]
pub struct ControlBar {}

impl Render for ControlBar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
    }
}

impl ControlBar {
    pub fn new() -> Self {
        ControlBar {}
    }
}
