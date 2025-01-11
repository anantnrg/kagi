use gpui::*;

#[derive(Clone)]
pub struct NowPlaying {
    pub title: SharedString,
    pub album: SharedString,
    pub artists: Vec<SharedString>,
}

pub enum NowPlayingEvent {
    Update(SharedString, SharedString, Vec<SharedString>),
}

impl NowPlaying {
    pub fn new() -> Self {
        NowPlaying {
            title: "".into(),
            artists: vec!["".into()],
            album: "".into(),
        }
    }

    pub fn update(
        &mut self,
        cx: &mut ModelContext<Self>,
        title: SharedString,
        album: SharedString,
        artists: Vec<SharedString>,
    ) {
        cx.emit(NowPlayingEvent::Update(title, album, artists));
        cx.notify();
    }
}

impl EventEmitter<NowPlayingEvent> for NowPlaying {}
