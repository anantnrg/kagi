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
        if window_width < 200.0 {
            div()
        } else {
            div()
                .bg(theme.background)
                .h_full()
                .w(px(window_width * (20.0 / 100.0)))
                .min_w(px(150.0))
                .border_r_1()
                .border_color(theme.secondary)
                .children(self.playlists.read(cx).into_iter().map(|(name, _)| {
                    div()
                        .w_full()
                        .mx_4()
                        .rounded_lg()
                        .h_10()
                        .flex()
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
