use super::{res_handler::*, titlebar::Titlebar};
use crate::{
    control_bar::ControlBar,
    layout::{Layout, LayoutMode},
    main_view::MainView,
    sidebar::*,
};
use components::theme::Theme;
use gpui::{prelude::FluentBuilder, *};

#[derive(Clone)]
pub struct Kagi {
    pub titlebar: Entity<Titlebar>,
    pub left_sidebar: Entity<LeftSidebar>,
    pub right_sidebar: Entity<RightSidebar>,
    pub control_bar: Entity<ControlBar>,
    pub main_view: Entity<MainView>,
    pub res_handler: Entity<ResHandler>,
}

impl Render for Kagi {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = self.clone().titlebar;
        let control_bar = self.clone().control_bar;
        let main_view = self.clone().main_view;
        let left_sidebar = self.clone().left_sidebar;
        let right_sidebar = self.clone().right_sidebar;
        let old_layout = cx.global::<Layout>().clone();

        // Recalculate layout
        *cx.global_mut::<Layout>() = old_layout.layout(win.bounds().size.width.0, cx);

        let theme = cx.global::<Theme>();
        let layout = cx.global::<Layout>();

        div()
            .bg(theme.bg)
            .w_full()
            .h_full()
            .font_family("Inter")
            .flex_col()
            .child(titlebar.clone())
            .child(
                div()
                    .w_full()
                    .h(px(
                        win.window_bounds().get_bounds().size.height.0 - (32.0 + 96.0)
                    ))
                    .flex()
                    .when(layout.mode.read(cx) == &LayoutMode::Inline, |this| {
                        this.px_3().gap_3()
                    })
                    .overflow_hidden()
                    .when(layout.left_sidebar.read(cx).clone().show, |this| {
                        this.child(left_sidebar)
                    })
                    .child(main_view)
                    .when(layout.right_sidebar.read(cx).clone().show, |this| {
                        this.child(right_sidebar)
                    }),
            )
            .child(control_bar)
    }
}
