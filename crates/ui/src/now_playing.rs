use gpui::*;

#[derive(Clone)]
pub struct NowPlaying {
    pub title: SharedString,
    pub album: SharedString,
    pub artists: Vec<SharedString>,
    pub position: u64,
    pub duration: u64,
    pub thumbnail: Option<ImageSource>,
}

pub enum NowPlayingEvent {
    Meta(SharedString, SharedString, Vec<SharedString>, u64),
    Position(u64),
    Thumbnail(ImageSource),
}

impl NowPlaying {
    pub fn new() -> Self {
        NowPlaying {
            title: "".into(),
            artists: vec!["".into()],
            album: "".into(),
            position: 0,
            duration: 0,
            thumbnail: None,
        }
    }

    pub fn update_meta(
        &mut self,
        cx: &mut ModelContext<Self>,
        title: SharedString,
        album: SharedString,
        artists: Vec<SharedString>,
        duration: u64,
    ) {
        cx.emit(NowPlayingEvent::Meta(title, album, artists, duration));
        cx.notify();
    }

    pub fn update_pos(&mut self, cx: &mut ModelContext<Self>, pos: u64) {
        cx.emit(NowPlayingEvent::Position(pos));
        cx.notify();
    }

    pub fn update_thumbnail(&mut self, cx: &mut ModelContext<Self>, image: ImageSource) {
        cx.emit(NowPlayingEvent::Thumbnail(image));
        cx.notify();
    }
}

impl EventEmitter<NowPlayingEvent> for NowPlaying {}
