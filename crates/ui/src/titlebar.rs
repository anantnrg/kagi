use crate::layout::Layout;
use crate::player_context::PlayerContext;
use components::theme::Theme;

use components::icon::*;
use gpui::*;
use prelude::FluentBuilder;

#[derive(Clone)]
pub struct Titlebar;

impl Render for Titlebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .w_full()
            .h_8()
            .bg(theme.titlebar.bg)
            .border_b_1()
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .pl(px(6.0))
                    .items_center()
                    .justify_start()
                    .child(
                        div()
                            .w_6()
                            .h_6()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .rounded_md()
                            .child(
                                Icon::new(Icons::LeftSidebar)
                                    .size(18.0)
                                    .color(theme.titlebar.icon.into()),
                            )
                            .on_mouse_down(MouseButton::Left, {
                                move |_, _, cx| {
                                    let layout = cx.global_mut::<Layout>().clone();
                                    layout.left_sidebar.update(cx, |this, cx| {
                                        this.should_show = !this.should_show.clone();
                                        cx.notify();
                                    });
                                }
                            }),
                    ),
            )
            .child(
                div()
                    .flex()
                    .w_full()
                    .h_full()
                    .px_4()
                    .items_center()
                    .justify_center()
                    .child({
                        let meta = cx.global::<PlayerContext>().metadata.read(cx);
                        let window_width = win.window_bounds().get_bounds().size.width.0;

                        let truncate = |text: &str, limit: usize| -> String {
                            if text.len() > limit {
                                format!("{}...", &text[..limit])
                            } else {
                                text.to_string()
                            }
                        };

                        div()
                            .text_color(theme.titlebar.title)
                            .text_sm()
                            .overflow_hidden()
                            .when(window_width < 200.0, |this| this.child("Kagi"))
                            .when((200.0..400.0).contains(&window_width), |this| {
                                this.child({
                                    if meta.title.is_empty() {
                                        "Kagi".to_string()
                                    } else {
                                        truncate(&meta.title, 30)
                                    }
                                })
                            })
                            .when((400.0..600.0).contains(&window_width), |this| {
                                if meta.title.is_empty() {
                                    this.child("No Song Playing".to_string())
                                } else {
                                    let artists = if meta.artists.is_empty() {
                                        "".to_string()
                                    } else {
                                        format!(" - {}", meta.artists.join(", "))
                                    };
                                    this.child(format!("{}{}", truncate(&meta.title, 30), artists))
                                }
                            })
                            .when(window_width >= 600.0, |this| {
                                if meta.title.is_empty() {
                                    this.child("Kagi".to_string())
                                } else {
                                    let artists = if meta.artists.is_empty() {
                                        "".to_string()
                                    } else {
                                        format!(" - {}", meta.artists.join(", "))
                                    };
                                    let album = if meta.album.is_empty() {
                                        "".to_string()
                                    } else {
                                        format!(" ({})", meta.album)
                                    };
                                    this.child(format!(
                                        "{}{}{}",
                                        truncate(&meta.title, 30),
                                        artists,
                                        album
                                    ))
                                }
                            })
                    }),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_end()
                    .w_full()
                    .h_full()
                    .child(
                        div()
                            .w_6()
                            .h_6()
                            .mr_2()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .rounded_md()
                            .child(
                                Icon::new(Icons::RightSidebar)
                                    .size(18.0)
                                    .color(theme.titlebar.icon.into()),
                            )
                            .on_mouse_down(MouseButton::Left, {
                                move |_, _, cx| {
                                    let layout = cx.global_mut::<Layout>().clone();
                                    layout.right_sidebar.update(cx, |this, cx| {
                                        this.should_show = !this.should_show.clone();
                                        cx.notify();
                                    });
                                }
                            }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .child(
                                Icon::new(Icons::Minimize)
                                    .size(20.0)
                                    .color(rgb(0xffffff))
                                    .hover(rgb(0xffffff)),
                            ),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0x45475a)))
                            .child({
                                if win.is_maximized() {
                                    Icon::new(Icons::Restore)
                                        .size(20.0)
                                        .color(rgb(0xffffff))
                                        .hover(rgb(0xffffff))
                                } else {
                                    Icon::new(Icons::Maximize)
                                        .size(20.0)
                                        .color(rgb(0xffffff))
                                        .hover(rgb(0xffffff))
                                }
                            }),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(rgb(0xe81123)))
                            .child(
                                Icon::new(Icons::Close)
                                    .size(20.0)
                                    .color(rgb(0xffffff))
                                    .hover(rgb(0xffffff)),
                            ),
                    ),
            )
    }
}

impl Titlebar {
    pub fn new() -> Titlebar {
        Titlebar {}
    }
}
