use gpui::*;

pub struct NowPlaying {
    pub title: SharedString,
    pub artists: Vec<SharedString>,
    pub album: SharedString,
}

pub enum NowPlayingEvent {
    Title(SharedString),
    Artists(Vec<SharedString>),
    Album(SharedString),
}

impl NowPlaying {
    pub fn new() -> Self {
        NowPlaying {
            title: "".into(),
            artists: vec!["".into()],
            album: "".into(),
        }
    }

    pub fn change_title(mut self, cx: &mut ViewContext<Self>, title: SharedString) -> Self {
        cx.emit(NowPlayingEvent::Title(title));
        cx.notify();
        self
    }
}

impl EventEmitter<NowPlayingEvent> for NowPlaying {}
