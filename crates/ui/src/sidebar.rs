use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

use crate::layout::{Layout, LayoutMode};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<Vec<(String, String)>>,
    pub active_index: Entity<usize>,
    pub layout: Entity<Layout>,
}

impl Render for LeftSidebar {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone();
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
                .children(
                    playlists
                        .into_iter()
                        .enumerate()
                        .map(|(index, (name, path))| {
                            let controller = controller.clone();
                            let curr_index = current_index.clone();
                            let current_index = *curr_index.read(cx);
                            let path = path.clone();

                            div()
                                .bg(theme.background)
                                .hover(|this| this.border_2().border_color(theme.accent))
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
                                .child(name.clone())
                                .on_mouse_down(MouseButton::Left, {
                                    move |_, _, cx| {
                                        curr_index.update(cx, |this, _| {
                                            *this = index;
                                        });
                                        controller.clone().load(path.clone());
                                        controller.clone().get_queue();
                                    }
                                })
                        }),
                )
                .child(
                    div()
                        .w_full()
                        .h_10()
                        .child("Open Folder")
                        .flex()
                        .items_center()
                        .justify_center()
                        .hover(|this| {
                            this.bg(theme.secondary)
                                .border_2()
                                .border_color(theme.accent)
                        })
                        .on_mouse_down(MouseButton::Left, move |_, _, _| {
                            controller.clone().open_folder();
                        }),
                )
        } else {
            div()
        }
    }
}

impl LeftSidebar {
    pub fn new(cx: &mut App, layout: Entity<Layout>) -> Self {
        LeftSidebar {
            playlists: cx.new(|_| {
                vec![(
                    "straight up liquid fire".to_string(),
                    "E:\\music\\straight up liquid fire".to_string(),
                )]
            }),
            active_index: cx.new(|_| 1),
            layout,
        }
    }
}
