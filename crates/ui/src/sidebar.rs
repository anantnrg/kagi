use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<Vec<(String, String, bool)>>,
     pub active_index: Entity<u32>
}

impl Render for LeftSidebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let window_width = win.window_bounds().get_bounds().size.width.0;
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone();
        if window_width < 400.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(window_width * (24.0 / 100.0)))
                .min_w(px(240.0))
                .border_r_1()
                .border_color(theme.secondary)
                .px_3()
                .children({
                    playlists.into_iter().map(|(name, path, current)| {
                        let controller = controller.clone();
                        let path = path.clone();

                        div()
                            .when(current.clone(), |this| this.bg(theme.secondary))
                            .bg(theme.background)
                            .text_color(theme.text)
                            .font_weight(FontWeight::MEDIUM)
                            .w_full()
                            .rounded_lg()
                            .h_10()
                            .flex()
                            .items_center()
                            .justify_start()
                            .px_2()
                            .child(name.clone())
                            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                controller.clone().load(path.clone());
                                self.playlists.update(cx, |this, cx| {
                                    this.
                                })
                            })
                    })
                })
        }
    }
}

impl LeftSidebar {
    pub fn new(cx: &mut App) -> Self {
        LeftSidebar {
            playlists: cx.new(|_| {
                vec![(
                    "straight up liquid fire".to_string(),
                    "E:\\music\\straight up liquid fire".to_string(),
                    false,
                )]
            }),
        }
    }
}
