use gpui::*;

pub struct NowPlaying {
    pub title: SharedString,
    pub artists: Vec<SharedString>,
    pub album: SharedString,
}
