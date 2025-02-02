use crate::now_playing::NowPlaying;
use components::theme::Theme;

use components::icon::*;
use gpui::*;
use prelude::FluentBuilder;

#[derive(Clone)]
pub struct Titlebar {
    now_playing: Entity<NowPlaying>,
}

impl Render for Titlebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let theme = cx.global::<Theme>();
        div()
            .w_full()
            .h_8()
            .bg(theme.titlebar_bg)
            .border_b_1()
            .border_color(theme.secondary)
            .flex()
            .items_center()
            .justify_between()
            .child(
                div()
                    .flex()
                    .w_6()
                    .h_6()
                    .ml(px(6.0))
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .hover(|this| this.bg(rgb(0x45475a)))
                    .child(
                        Icon::new(Icons::LeftSidebar)
                            .size(18.0)
                            .color(theme.icon.into()),
                    ),
            )
            .child(div().flex().w_auto().h_full().items_center().child({
                let np = self.now_playing.read(cx);
                let window_width = win.window_bounds().get_bounds().size.width.0;

                let truncate = |text: &str, limit: usize| -> String {
                    if text.len() > limit {
                        format!("{}...", &text[..limit])
                    } else {
                        text.to_string()
                    }
                };

                div()
                    .text_color(theme.accent)
                    .text_sm()
                    .overflow_hidden()
                    .when(window_width < 200.0, |this| this.child("Reyvr"))
                    .when((200.0..400.0).contains(&window_width), |this| {
                        this.child({
                            if np.title.is_empty() {
                                "Reyvr".to_string()
                            } else {
                                truncate(&np.title, 30)
                            }
                        })
                    })
                    .when((400.0..600.0).contains(&window_width), |this| {
                        if np.title.is_empty() {
                            this.child("No Song Playing".to_string())
                        } else {
                            let artists = if np.artists.is_empty() {
                                "".to_string()
                            } else {
                                format!(" - {}", np.artists.join(", "))
                            };
                            this.child(format!("{}{}", truncate(&np.title, 30), artists))
                        }
                    })
                    .when(window_width >= 600.0, |this| {
                        if np.title.is_empty() {
                            this.child("Reyvr".to_string())
                        } else {
                            let artists = if np.artists.is_empty() {
                                "".to_string()
                            } else {
                                format!(" - {}", np.artists.join(", "))
                            };
                            let album = if np.album.is_empty() {
                                "".to_string()
                            } else {
                                format!(" ({})", np.album)
                            };
                            this.child(format!("{}{}{}", truncate(&np.title, 30), artists, album))
                        }
                    })
            }))
            .child(
                div()
                    .flex()
                    .items_center()
                    .w_auto()
                    .h_full()
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(theme.secondary))
                            .child(Icon::new(Icons::Minimize).size(20.0).color(0xffffff)),
                    )
                    .child(
                        div()
                            .w(px(36.0))
                            .h_full()
                            .flex()
                            .items_center()
                            .justify_center()
                            .hover(|this| this.bg(theme.secondary))
                            .child({
                                if win.is_maximized() {
                                    Icon::new(Icons::Restore).size(20.0).color(0xffffff)
                                } else {
                                    Icon::new(Icons::Maximize).size(20.0).color(0xffffff)
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
                            .child(Icon::new(Icons::Close).size(20.0).color(0xffffff)),
                    ),
            )
    }
}

impl Titlebar {
    pub fn new(now_playing: Entity<NowPlaying>) -> Titlebar {
        Titlebar { now_playing }
    }
}
