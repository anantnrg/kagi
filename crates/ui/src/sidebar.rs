use backend::{playback::SavedPlaylists, player::Controller};
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

use crate::layout::{Layout, LayoutMode};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<SavedPlaylists>,
    pub active_index: Entity<usize>,
    pub layout: Entity<Layout>,
}

impl Render for LeftSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone().playlists;
        let current_index = self.active_index.clone();
        let layout = self.layout.clone().read(cx);

        if layout.left_sidebar.show {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(layout.left_sidebar.width))
                .min_w(px(200.0))
                .when(layout.mode == LayoutMode::Overlay, |this| {
                    this.absolute().border_0()
                })
                .border_r_1()
                .border_color(theme.secondary)
                .px_3()
                .py_3()
                .flex()
                .flex_col()
                .gap_2()
                .children(playlists.into_iter().enumerate().map(|(index, playlist)| {
                    let controller = controller.clone();
                    let curr_index = current_index.clone();
                    let current_index = *curr_index.read(cx);
                    let path = playlist.actual_path.clone();

                    div()
                        .bg(theme.background)
                        .border_1()
                        .border_color(theme.secondary)
                        .hover(|this| this.border_color(theme.accent))
                        .when(index == current_index, |this| this.bg(theme.secondary))
                        .text_color(theme.text)
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
                                    *this = index;
                                });
                                controller.load(path.clone());
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
                        .text_color(theme.text)
                        .border_1()
                        .border_color(theme.secondary)
                        .hover(|this| this.bg(theme.secondary).border_color(theme.accent))
                        .on_mouse_down(MouseButton::Left, move |_, _, _| {
                            controller.open_folder();
                            controller.get_queue();
                            controller.write_playlist();
                            controller.retrieve_saved_playlists();
                        }),
                )
        } else {
            div()
        }
    }
}

impl LeftSidebar {
    pub fn new(cx: &mut App, playlists: Entity<SavedPlaylists>, layout: Entity<Layout>) -> Self {
        LeftSidebar {
            playlists,
            active_index: cx.new(|_| 1),
            layout,
        }
    }
}
