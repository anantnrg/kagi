use components::theme::Theme;
use gpui::*;

#[derive(Clone)]
pub struct LeftSidebar {
    pub playlists: Entity<Vec<(String, String)>>,
}

impl Render for LeftSidebar {
    fn render(&mut self, win: &mut Window, cx: &mut Context<'_, Self>) -> impl IntoElement {
        let window_width = win.window_bounds().get_bounds().size.width.0;
        let theme = cx.global::<Theme>();
        if window_width < 400.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(window_width * (30.0 / 100.0)))
                .min_w(px(240.0))
                .border_r_1()
                .border_color(theme.secondary)
                .children(self.playlists.read(cx).into_iter().map(|(name, _)| {
                    div()
                        .bg(theme.secondary)
                        .w_full()
                        .mx_2()
                        .rounded_lg()
                        .h_10()
                        .flex()
                        .items_start()
                        .px_2()
                        .child(name.clone())
                }))
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
        }
    }
}
