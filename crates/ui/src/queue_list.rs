use backend::player::Controller;
use components::theme::Theme;
use gpui::*;

use crate::now_playing::NowPlaying;

pub struct QueueList {
    pub now_playing: Entity<NowPlaying>,
    pub scroll_offset: Entity<f32>,
}

impl Render for QueueList {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let window_width = win.window_bounds().get_bounds().size.width.0;
        let window_height = win.window_bounds().get_bounds().size.height.0;
        let tracks = self.now_playing.read(cx).tracks.clone();

        let item_height = 56.0_f32;
        let total_items = tracks.len();
        let visible_count = ((window_height - 32.0) / item_height).ceil() as usize;
        let first_visible_index = (self.scroll_offset.read(cx) / item_height).floor() as usize;
        let start_index = first_visible_index.saturating_sub(2);
        let end_index = usize::min(total_items, first_visible_index + visible_count + 2);

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
                .children(tracks[start_index..end_index].into_iter().enumerate().map(
                    |(id, track)| {
                        div()
                            .w_full()
                            .h_16()
                            .flex()
                            .px_3()
                            .gap_2()
                            .text_color(theme.text)
                            .items_center()
                            .justify_between()
                            .px_2()
                            .border_b_1()
                            .border_color(theme.secondary)
                            .rounded_md()
                            .hover(|this| this.bg(theme.secondary))
                            .on_mouse_down(MouseButton::Left, move |_, _, cx| {
                                let controller = cx.global::<Controller>().clone();
                                controller.play_id(id);
                            })
                            .child({
                                if let Some(thumbnail) = track.thumbnail.clone() {
                                    img(ImageSource::Render(
                                        RenderImage::new(thumbnail.img.clone()).into(),
                                    ))
                                    .h(px(56.0))
                                    .rounded_md()
                                } else {
                                    img("")
                                }
                            })
                            .child(
                                div()
                                    .w_full()
                                    .h(px(56.0))
                                    .flex()
                                    .flex_col()
                                    .gap(px(1.0))
                                    .child(
                                        div()
                                            .child(track.title.clone())
                                            .truncate()
                                            .text_ellipsis()
                                            .text_base()
                                            .font_weight(FontWeight::MEDIUM),
                                    )
                                    .child(
                                        div()
                                            .child(track.artists.join(", "))
                                            .truncate()
                                            .text_ellipsis()
                                            .text_sm()
                                            .font_weight(FontWeight::NORMAL),
                                    ),
                            )
                    },
                ))
        }
    }
}

impl QueueList {
    pub fn new(now_playing: Entity<NowPlaying>, scroll_offset: Entity<f32>) -> Self {
        QueueList {
            now_playing,
            scroll_offset,
        }
    }
}
