use components::theme::Theme;
use gpui::*;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct ControlBar {
    now_playing: NowPlaying,
}

impl Render for ControlBar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .w_full()
            .h(px(72.0))
            .border_t_1()
            .bg(theme.background)
            .border_color(theme.secondary)
            .flex()
            .items_center()
            .justify_center()
            .child(div().size_5().child(svg().path("assets/")))
    }
}

impl ControlBar {
    pub fn new(now_playing: NowPlaying) -> Self {
        ControlBar { now_playing }
    }
}
