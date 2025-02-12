use std::sync::{Arc, Mutex};

use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};
use simsearch::SimSearch;

use crate::{
    layout::{Layout, LayoutMode},
    now_playing::{NowPlaying, Track},
};

pub struct QueueList {
    pub now_playing: Entity<NowPlaying>,
    pub layout: Entity<Layout>,
    pub simsearch: SimSearch<String>,
}

impl Render for QueueList {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let query = cx.new(|_| String::from("miss"));

        let theme = cx.global::<Theme>();
        let layout = self.layout.clone().read(cx);
        let tracks = self.search(
            self.now_playing.read(cx).tracks.clone(),
            query.read(cx).clone(),
        );
        for track in tracks.clone() {
            println!("{:#?}", track.title);
        }

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
                .child(
                    uniform_list(
                        cx.entity(),
                        "queue_list",
                        tracks.len(),
                        move |_, range, _, cx| {
                            let theme = cx.global::<Theme>();

                            range
                                .map(|id| {
                                    let track = &tracks[id];

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
                                            if let Some(thumbnail) = &track.thumbnail {
                                                img(thumbnail.img.clone())
                                                    .min_h(px(56.0))
                                                    .min_w(px(56.0))
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
                                })
                                .collect()
                        },
                    )
                    .h_full(),
                )
        } else {
            div()
        }
    }
}

impl QueueList {
    pub fn new(now_playing: Entity<NowPlaying>, layout: Entity<Layout>) -> Self {
        let simsearch = SimSearch::new();
        QueueList {
            now_playing,
            layout,
            simsearch,
        }
    }

    pub fn search(&mut self, tracks: Vec<Track>, query: String) -> Vec<Track> {
        for track in &tracks {
            let key = format!(
                "{} {} {}",
                track.title,
                track.artists.join(", "),
                track.album
            );

            self.simsearch.insert(key.clone(), &key);
        }

        let results = self.simsearch.search(query.as_str());

        tracks
            .iter()
            .filter(|track| {
                let key = format!(
                    "{} {} {}",
                    track.title,
                    track.artists.join(", "),
                    track.album
                );
                results.contains(&key.clone())
            })
            .cloned()
            .collect()
    }
}
