use backend::player::Controller;
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<Vec<(String, String)>>,
    pub active_index: Entity<usize>,
}

impl Render for LeftSidebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let window_width = win.window_bounds().get_bounds().size.width.0;
        let theme = cx.global::<Theme>();
        let controller = cx.global::<Controller>().clone();
        let playlists = self.playlists.read(cx).clone();
        let current_index = *self.active_index.clone().read(cx);

        if window_width < 400.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w_1_5()
                .min_w(px(240.0))
                .border_r_1()
                .border_color(theme.secondary)
                .px_3()
                .children(
                    playlists
                        .into_iter()
                        .enumerate()
                        .map(|(index, (name, path))| {
                            let controller = controller.clone();
                            let path = path.clone();

                            div()
                                .when(current_index == index, |this| this.bg(theme.secondary))
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
                                .on_mouse_down(MouseButton::Left, {
                                    move |_, _, _| {
                                        controller.clone().load(path.clone());
                                        controller.clone().get_queue();
                                    }
                                })
                        }),
                )
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
                )]
            }),
            active_index: cx.new(|_| 0),
        }
    }
}
