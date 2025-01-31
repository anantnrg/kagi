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
        let np = self.now_playing.clone();
        if window_width < 600.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w_1_3()
                .min_w(px(320.0))
                .border_l_1()
                .border_color(theme.secondary)
                .children(tracks.into_iter().enumerate().map(|(id, track)| {
                    div()
                        .w_full()
                        .h_16()
                        .flex()
                        .px_3()
                        .text_color(theme.text)
                        .items_center()
                        .justify_between()
                        .px_2()
                        .border_b_1()
                        .border_color(theme.secondary)
                        .child({
                            if let Some(thumbnail) = np.read(cx).thumbnail.clone() {
                                img(thumbnail.img).size_12().rounded_lg()
                            } else {
                                img("")
                            }
                        })
                        .child(
                            div()
                                .w_full()
                                .h_full()
                                .flex()
                                .flex_col()
                                .gap_2()
                                .child(
                                    div()
                                        .child(track.title)
                                        .truncate()
                                        .text_base()
                                        .font_weight(FontWeight::MEDIUM),
                                )
                                .child(
                                    div()
                                        .child(track.artists.join(", "))
                                        .truncate()
                                        .text_sm()
                                        .font_weight(FontWeight::NORMAL),
                                ),
                        )
                }))
        }
    }
}

impl QueueList {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        QueueList { now_playing }
    }
}
