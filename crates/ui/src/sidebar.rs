use backend::{playback::SavedPlaylists, player::Controller};
use components::{input::TextInput, theme::Theme};
use gpui::{prelude::FluentBuilder, *};
use nucleo::{
    Config, Nucleo,
    pattern::{CaseMatching, Normalization},
};
use std::collections::HashSet;
use std::sync::Arc;

use crate::{
    layout::{Layout, LayoutMode},
    player_context::{PlayerContext, Track},
};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<SavedPlaylists>,
}

pub struct RightSidebar {
    pub nucleo: Nucleo<(usize, String)>,
    pub query: Entity<String>,
    pub tracks: Vec<Track>,
    text_input: Entity<TextInput>,
    focus_handle: FocusHandle,
}

impl Focusable for RightSidebar {
    fn focus_handle(&self, _: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for LeftSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone().playlists;
        let current_index = cx.global::<PlayerContext>().metadata.clone();
        let layout = cx.global::<Layout>().clone();

        if layout.left_sidebar.read(cx).clone().show {
            deferred(
                div()
                    .track_focus(&cx.focus_handle())
                    .bg(theme.left_sidebar.bg)
                    .h_full()
                    .w(px(layout.left_sidebar.read(cx).clone().width))
                    .min_w(px(200.0))
                    .when(
                        layout.mode.read(cx).clone() == LayoutMode::Overlay,
                        |this| this.absolute().bg(theme.bg).rounded_none(),
                    )
                    .occlude()
                    .border_r_1()
                    .px_3()
                    .py_3()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .children(playlists.into_iter().map(|playlist| {
                        let controller = controller.clone();
                        let curr_index = current_index.clone();
                        let current_index = curr_index.read(cx).playlist_name.clone();

                        div()
                            .bg(theme.left_sidebar.bg)
                            .border_1()
                            .border_color(theme.left_sidebar.item_border)
                            .hover(|this| this.border_color(theme.left_sidebar.item_hover))
                            .when(playlist.name == current_index.clone(), |this| {
                                this.bg(theme.left_sidebar.item_bg)
                            })
                            .text_color(theme.left_sidebar.item_text)
                            .font_weight(FontWeight::MEDIUM)
                            .w_full()
                            .rounded_lg()
                            .h_10()
                            .flex()
                            .items_center()
                            .justify_start()
                            .px_3()
                            .child(playlist.name.clone())
                            .truncate()
                            .on_mouse_down(MouseButton::Left, {
                                move |_, _, cx| {
                                    curr_index.update(cx, |this, _| {
                                        this.playlist_name = playlist.name.clone().into();
                                    });
                                    controller.load(playlist.clone());
                                    controller.get_queue();
                                }
                            })
                    }))
                    .child(
                        div()
                            .w_full()
                            .h_10()
                            .child("Open Folder")
                            .flex()
                            .items_center()
                            .justify_center()
                            .rounded_lg()
                            .text_color(theme.left_sidebar.item_text)
                            .border_1()
                            .border_color(theme.left_sidebar.item_border)
                            .hover(|this| this.bg(theme.left_sidebar.item_bg))
                            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                controller.open_folder();
                                controller.get_queue();
                                controller.write_playlist();
                                controller.retrieve_saved_playlists();
                            }),
                    ),
            )
            .with_priority(1)
        } else {
            deferred(div().track_focus(&cx.focus_handle())).with_priority(1)
        }
    }
}

impl Render for RightSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tracks = cx.global::<PlayerContext>().tracks.clone();
        let tracks = self.search(tracks.read(cx).clone(), self.query.read(cx).clone());

        let theme = cx.global::<Theme>();
        let layout = cx.global::<Layout>().clone();

        if layout.right_sidebar.read(cx).clone().show {
            deferred(
                div()
                    .track_focus(&cx.focus_handle())
                    .bg(theme.right_sidebar.bg)
                    .h_full()
                    .w(px(layout.right_sidebar.read(cx).clone().width))
                    .flex()
                    .flex_col()
                    .min_w(px(280.0))
                    .when(
                        layout.mode.read(cx).clone() == LayoutMode::Overlay,
                        |this| this.absolute().border_0(),
                    )
                    .border_l_1()
                    .occlude()
                    .child(
                        div()
                            .w_full()
                            .h_10()
                            .py_1()
                            .px_1()
                            .border_b_1()
                            .child(self.text_input.clone()),
                    )
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
                                            .mt_2()
                                            .gap_2()
                                            .text_color(theme.right_sidebar.item_title)
                                            .items_center()
                                            .justify_between()
                                            .px_2()
                                            .rounded_lg()
                                            .overflow_hidden()
                                            .hover(|this| this.bg(theme.right_sidebar.item_hover))
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
                        .h_full()
                        .px_1(),
                    ),
            )
            .with_priority(2)
        } else {
            deferred(div().track_focus(&cx.focus_handle())).with_priority(2)
        }
    }
}

impl LeftSidebar {
    pub fn new(playlists: Entity<SavedPlaylists>) -> Self {
        LeftSidebar { playlists }
    }
}

impl RightSidebar {
    pub fn new(cx: &mut Context<RightSidebar>) -> Self {
        let query = cx.new(|_| String::new());
        let handle = cx.focus_handle();

        let nucleo: Nucleo<(usize, String)> =
            Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);

        let text_input = TextInput::new(cx, handle, None, Some("Search something...".into()));
        let query_clone = query.clone();
        cx.subscribe(&text_input, move |_: &mut RightSidebar, _, text, cx| {
            query_clone.update(cx, |this, _| {
                *this = text.to_string();
            });
            cx.notify();
        })
        .detach();

        RightSidebar {
            nucleo,
            query,
            tracks: vec![],
            text_input,
            focus_handle: cx.focus_handle(),
        }
    }

    pub fn search(&mut self, tracks: Vec<Track>, query: String) -> Vec<Track> {
        if self.tracks != tracks {
            self.nucleo = Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1);
            let injector = self.nucleo.injector();

            for (i, track) in tracks.iter().enumerate() {
                let key = format!(
                    "{} {} {}",
                    track.title,
                    track.artists.join(", "),
                    track.album
                );
                injector.push((i, key.clone()), |&(_id, ref string), row| {
                    row[0] = string.as_str().into();
                });
            }

            self.tracks = tracks;
        }

        if query.trim().is_empty() {
            return self.tracks.clone();
        }

        self.nucleo
            .pattern
            .reparse(0, &query, CaseMatching::Ignore, Normalization::Smart, false);

        self.nucleo.tick(500);

        let snapshot = self.nucleo.snapshot();
        let results: HashSet<usize> = snapshot.matched_items(..).map(|item| item.data.0).collect();
        let mut filtered_tracks: Vec<Track> = self
            .tracks
            .clone()
            .into_iter()
            .enumerate()
            .filter_map(|(index, track)| {
                if results.contains(&index) {
                    Some(track)
                } else {
                    None
                }
            })
            .collect();
        filtered_tracks.reverse();

        filtered_tracks
    }
}
