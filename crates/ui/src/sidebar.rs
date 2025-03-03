use backend::{
    playback::{SavedPlaylist, SavedPlaylists},
    player::Controller,
};
use components::{icon::Icon, input::TextInput, theme::Theme};
use gpui::{prelude::FluentBuilder, *};
use nucleo::{
    Config, Nucleo,
    pattern::{CaseMatching, Normalization},
};
use std::collections::HashSet;
use std::{path::PathBuf, sync::Arc};

use crate::{
    layout::{Layout, LayoutMode},
    player_context::{PlayerContext, Track},
};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<SavedPlaylists>,
}

pub struct LeftSidebarItem {
    playlist: SavedPlaylist,
    // hovered: bool,
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
        let theme = cx.global::<Theme>().clone();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone().playlists;
        let layout = cx.global::<Layout>().clone();

        deferred(
            div()
                .track_focus(&cx.focus_handle())
                .bg(theme.left_sidebar.bg)
                .h_full()
                .w(px(layout.left_sidebar.read(cx).clone().width))
                .min_w(px(200.0))
                .rounded_xl()
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
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.left_sidebar.title)
                        .w_full()
                        .h_auto()
                        .flex()
                        .items_center()
                        .gap_x_2()
                        .text_center()
                        .mb_2()
                        .child(
                            Icon::new(components::icon::Icons::Library)
                                .size(20.0)
                                .color(theme.left_sidebar.title)
                                .hover(theme.left_sidebar.title),
                        )
                        .child("Library"),
                )
                .children(
                    playlists
                        .into_iter()
                        .map(|playlist| cx.new(|_| LeftSidebarItem::new(playlist))),
                )
                .child(
                    div()
                        .w_full()
                        .h_10()
                        .child("Open a folder...")
                        .flex()
                        .items_center()
                        .justify_center()
                        .rounded_lg()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
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
    }
}

impl Render for LeftSidebarItem {
    fn render(&mut self, _: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let playlist = self.playlist.clone();
        let controller = cx.global::<Controller>().clone();
        let index_write = cx.global::<PlayerContext>().metadata.clone();
        let index = cx
            .global::<PlayerContext>()
            .metadata
            .read(cx)
            .playlist_name
            .clone();
        let theme = cx.global::<Theme>();
        let thumbnail_path = PathBuf::from(playlist.actual_path.clone()).join("thumbnail.png");
        div()
            .bg(theme.left_sidebar.bg)
            .border_1()
            .border_color(theme.left_sidebar.item_border)
            .hover(|this| this.bg(theme.left_sidebar.item_hover))
            .when(playlist.name == index.clone(), |this| {
                this.bg(theme.left_sidebar.item_hover)
            })
            .text_color(theme.left_sidebar.item_text)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .w_full()
            .rounded_lg()
            .h_16()
            .flex()
            .items_center()
            .justify_start()
            .gap_2()
            .px_1()
            .child(
                img(thumbnail_path)
                    .min_w(px(56.0))
                    .max_w(px(56.0))
                    .rounded_md(),
            )
            .child(playlist.name.clone())
            .truncate()
            .on_mouse_down(MouseButton::Left, {
                move |_, _, cx| {
                    index_write.update(cx, |this, _| {
                        this.playlist_name = playlist.name.clone().into();
                    });
                    controller.load(playlist.clone());
                    controller.get_queue();
                }
            })
    }
}

impl LeftSidebarItem {
    pub fn new(playlist: SavedPlaylist) -> Self {
        LeftSidebarItem {
            playlist,
            // hovered: false,
        }
    }
}

impl Render for RightSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let tracks = cx.global::<PlayerContext>().tracks.clone();
        let tracks = self.search(tracks.read(cx).clone(), self.query.read(cx).clone());

        let theme = cx.global::<Theme>();
        let layout = cx.global::<Layout>().clone();

        deferred(
            div()
                .track_focus(&cx.focus_handle())
                .bg(theme.right_sidebar.bg)
                .h_full()
                .w(px(layout.right_sidebar.read(cx).clone().width))
                .flex()
                .flex_col()
                .min_w(px(280.0))
                .max_w(px(420.0))
                .rounded_xl()
                .when(
                    layout.mode.read(cx).clone() == LayoutMode::Overlay,
                    |this| this.absolute().border_0().max_w_full(),
                )
                .border_l_1()
                .occlude()
                .px_3()
                .py_3()
                .gap_2()
                .child(
                    div()
                        .font_weight(FontWeight::BOLD)
                        .text_color(theme.left_sidebar.title)
                        .w_full()
                        .h_auto()
                        .flex()
                        .items_center()
                        .gap_x_2()
                        .text_center()
                        .mb_2()
                        .child(
                            Icon::new(components::icon::Icons::Playlist)
                                .size(20.0)
                                .color(theme.left_sidebar.title)
                                .hover(theme.left_sidebar.title),
                        )
                        .child("Playlist"),
                )
                .child(
                    div()
                        .w_full()
                        .h_12()
                        .px_3()
                        .border_1()
                        .border_color(theme.right_sidebar.item_border)
                        .bg(theme.right_sidebar.search_bg)
                        .rounded_xl()
                        .text_sm()
                        .flex()
                        .gap_x_2()
                        .items_center()
                        .overflow_hidden()
                        .child(
                            Icon::new(components::icon::Icons::Search)
                                .size(20.0)
                                .color(theme.right_sidebar.search_text)
                                .hover(theme.right_sidebar.search_text),
                        )
                        .child(div().w_full().max_w_full().child(self.text_input.clone())),
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
                                        .h(px(64.0))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .child(
                                            div()
                                                .w_full()
                                                .h(px(56.0))
                                                .flex()
                                                .mt_2()
                                                .gap_2()
                                                .text_color(theme.right_sidebar.item_title)
                                                .items_center()
                                                .justify_between()
                                                .px(px(6.0))
                                                .rounded_lg()
                                                .overflow_hidden()
                                                .border_1()
                                                .border_color(theme.right_sidebar.item_border)
                                                .hover(|this| {
                                                    this.bg(theme.right_sidebar.item_hover)
                                                })
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    move |_, _, cx| {
                                                        let controller =
                                                            cx.global::<Controller>().clone();
                                                        controller.play_id(id);
                                                    },
                                                )
                                                .child({
                                                    if let Some(thumbnail) = &track.thumbnail {
                                                        img(thumbnail.img.clone())
                                                            .min_h(px(46.0))
                                                            .min_w(px(46.0))
                                                            .h(px(46.0))
                                                            .w(px(46.0))
                                                            .rounded_md()
                                                    } else {
                                                        img("")
                                                    }
                                                })
                                                .child(
                                                    div()
                                                        .w_full()
                                                        .flex()
                                                        .flex_col()
                                                        .gap(px(1.0))
                                                        .child(
                                                            div()
                                                                .child(track.title.clone())
                                                                .truncate()
                                                                .text_ellipsis()
                                                                .text_base()
                                                                .text_sm()
                                                                .font_weight(FontWeight::NORMAL),
                                                        )
                                                        .child(
                                                            div()
                                                                .child(track.artists.join(", "))
                                                                .truncate()
                                                                .text_ellipsis()
                                                                .text_xs()
                                                                .font_weight(FontWeight::NORMAL),
                                                        ),
                                                ),
                                        )
                                })
                                .collect()
                        },
                    )
                    .h_full(),
                ),
        )
        .with_priority(2)
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
