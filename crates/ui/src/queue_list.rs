use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

use crate::{
    layout::{Layout, LayoutMode},
    now_playing::NowPlaying,
};

pub struct QueueList {
    pub now_playing: Entity<NowPlaying>,
    pub layout: Entity<Layout>,
}

impl Render for QueueList {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let layout = self.layout.clone().read(cx);
        let tracks = self.now_playing.read(cx).tracks.clone();

        if layout.right_sidebar.show {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(layout.right_sidebar.width))
                .min_w(px(280.0))
                .when(layout.mode == LayoutMode::Overlay, |this| {
                    this.absolute().border_0()
                })
                .border_l_1()
                .border_color(theme.secondary)
                .id("scrollview")
                .overflow_y_scroll()
                .children(tracks.into_iter().enumerate().map(|(id, track)| {
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
                }))
        } else {
            div().id("")
        }
    }
}

impl QueueList {
    pub fn new(now_playing: Entity<NowPlaying>, layout: Entity<Layout>) -> Self {
        QueueList {
            now_playing,
            layout,
        }
    }
}
