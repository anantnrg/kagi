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
            .h_24()
            .border_t_1()
            .border_color(theme.secondary)
    }
}

impl ControlBar {
    pub fn new(now_playing: NowPlaying) -> Self {
        ControlBar { now_playing }
    }
}
