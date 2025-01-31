use backend::player::Controller;
use components::theme::Theme;
use gpui::*;

use crate::now_playing::NowPlaying;

pub struct QueueList {
    pub now_playing: Entity<NowPlaying>,
}

impl Render for QueueList {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let window_width = win.window_bounds().get_bounds().size.width.0;
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let tracks = self.now_playing.read(cx).tracks.clone();
        if window_width < 600.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(window_width * 0.32))
                .min_w(px(320.0))
                .border_l_1()
                .border_color(theme.secondary)
                .px_3()
                .children(tracks.into_iter().enumerate().map(|(id, track)| {
                    div()
                        .w_full()
                        .h_12()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_2()
                        .border_b_1()
                        .border_color(theme.secondary)
                        .child(id.to_string())
                        .child(track.title)
                }))
        }
    }
}

impl QueueList {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        QueueList { now_playing }
    }
}
