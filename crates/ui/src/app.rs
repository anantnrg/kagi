use super::{res_handler::*, titlebar::Titlebar};
use crate::{
    control_bar::ControlBar, layout::Layout, main_view::MainView, queue_list::QueueList, sidebar::*,
};
use components::theme::Theme;
use gpui::*;

#[derive(Clone)]
pub struct Kagi {
    pub titlebar: Entity<Titlebar>,
    pub left_sidebar: Entity<LeftSidebar>,
    pub queue_list: Entity<QueueList>,
    pub control_bar: Entity<ControlBar>,
    pub main_view: Entity<MainView>,
    pub res_handler: Entity<ResHandler>,
}

impl Render for Kagi {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = self.clone().titlebar;
        let sidebar = self.clone().left_sidebar;
        let control_bar = self.clone().control_bar;
        let main_view = self.clone().main_view;
        let queue_list = self.clone().queue_list;
        let old_layout = cx.global::<Layout>().clone();
        println!("{:#?}", old_layout.left_sidebar.read(cx).clone());
        *cx.global_mut::<Layout>() = old_layout.layout(win.bounds().size.width.0, cx);
        // let new_layout = {
        //     let layout_clone = cx.global::<Layout>().clone();
        //     layout_clone.layout(win.bounds().size.width.0, cx)
        // };
        // {
        //     let layout = cx.global_mut::<Layout>();
        //     *layout = new_layout;
        // }
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h_full()
            .flex_col()
            .child(titlebar.clone())
            .child(
                div()
                    .w_full()
                    .h(px(
                        win.window_bounds().get_bounds().size.height.0 - (32.0 + 96.0)
                    ))
                    .bg(theme.background)
                    .flex()
                    .overflow_hidden()
                    .child(sidebar)
                    .child(main_view)
                    .child(queue_list),
            )
            .child(control_bar)
    }
}
