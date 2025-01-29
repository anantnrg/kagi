use super::{now_playing::*, res_handler::*, titlebar::Titlebar};
use crate::{control_bar::ControlBar, layout::Layout, sidebar::*};
use components::theme::Theme;
use gpui::*;

#[derive(Clone)]
pub struct Reyvr {
    pub titlebar: Entity<Titlebar>,
    pub left_sidebar: Entity<LeftSidebar>,
    pub control_bar: Entity<ControlBar>,
    pub layout: Layout,
    pub now_playing: Entity<NowPlaying>,
    pub res_handler: Entity<ResHandler>,
}

impl Render for Reyvr {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let titlebar = self.clone().titlebar;
        let sidebar = self.clone().left_sidebar;
        let control_bar = self.clone().control_bar;
        let theme = cx.global::<Theme>();

        div()
            .w_full()
            .h_full()
            .flex_col()
            .child(titlebar.clone())
            .child(
                // div()
                //     .flex()
                //     .flex_wrap()
                //     .gap_8()
                //     .bg(theme.background)
                //     .size_full()
                //     .justify_center()
                //     .items_center()
                //     .child(Button::new().text("Play").on_click({
                //         move |_, _, cx| {
                //             cx.global::<Controller>().play();
                //         }
                //     }))
                //     .child(Button::new().text("Pause").on_click({
                //         move |_, _, cx| {
                //             cx.global::<Controller>().pause();
                //         }
                //     }))
                //     .child(Button::new().text("Previous").on_click({
                //         move |_, _, cx| {
                //             cx.global::<Controller>().prev();
                //         }
                //     }))
                //     .child(Button::new().text("Next").on_click({
                //         move |_, _, cx| {
                //             cx.global::<Controller>().next();
                //         }
                //     }))
                //     .child(Button::new().text("Load Playlist").on_click({
                //         move |_, _, cx| {
                //             cx.global::<Controller>()
                //                 .load("E:\\music\\straight up liquid fire");
                //         }
                //     }))

                //     .child({
                //         if let Some(thumbnail) = self.now_playing.read(cx).thumbnail.clone() {
                //             div()
                //                 .w(px(1280.))
                //                 .h(px(768.0))
                //                 .child(img(thumbnail).size_full())
                //         } else {
                //             div().size_128()
                //         }
                //     }),
                div()
                    .w_full()
                    .h(px(
                        win.window_bounds().get_bounds().size.height.0 - (32.0 + 72.0)
                    ))
                    .bg(theme.background)
                    .flex()
                    .overflow_hidden()
                    .child(sidebar),
            )
            .child(control_bar)
    }
}
