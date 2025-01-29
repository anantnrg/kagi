use gpui::*;

use crate::now_playing::NowPlaying;

#[derive(Clone)]
pub struct MainView {
    pub now_playing: Entity<NowPlaying>,
}

impl Render for MainView {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let np = self.now_playing.clone();
        div()
            .w_full()
            .h_full()
            .flex()
            .items_center()
            .justify_center()
            .flex_col()
            .child({
                if let Some(thumbnail) = np.read(cx).thumbnail.clone() {
                    div()
                        .w(px(1280.))
                        .h(px(768.0))
                        .child(img(thumbnail).size_full())
                } else {
                    div()
                }
            })
    }
}

impl MainView {
    pub fn new(now_playing: Entity<NowPlaying>) -> Self {
        MainView { now_playing }
    }
}
