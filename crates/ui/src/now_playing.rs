use gpui::*;

#[derive(Clone)]
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

    pub fn change_title(&mut self, cx: &mut ModelContext<Self>, title: SharedString) {
        cx.emit(NowPlayingEvent::Title(title));
        cx.notify();
    }
}

impl EventEmitter<NowPlayingEvent> for NowPlaying {}
