use gpui::*;
use gstreamer::State;

#[derive(Clone)]
pub struct NowPlaying {
    pub playlist_name: SharedString,
    pub title: SharedString,
    pub album: SharedString,
    pub artists: Vec<SharedString>,
    pub position: u64,
    pub duration: u64,
    pub thumbnail: Option<Thumbnail>,
    pub state: State,
    pub volume: f64,
    pub tracks: Vec<Track>,
}

#[derive(Clone)]
pub struct Thumbnail {
    pub img: ImageSource,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub uri: String,
    pub duration: u64,
    pub thumbnail: Option<Thumbnail>,
}

pub enum NowPlayingEvent {
    Meta(SharedString, SharedString, Vec<SharedString>, u64),
    Position(u64),
    Thumbnail(Thumbnail),
    State(State),
    Volume(f64),
    Tracks(Vec<Track>),
}

impl NowPlaying {
    pub fn new() -> Self {
        NowPlaying {
            playlist_name: "".into(),
            title: "".into(),
            artists: vec!["".into()],
            album: "".into(),
            position: 0,
            duration: 0,
            thumbnail: None,
            state: State::Null,
            volume: 0.2,
            tracks: vec![],
        }
    }

    pub fn update_meta(
        &mut self,
        cx: &mut Context<Self>,
        title: SharedString,
        album: SharedString,
        artists: Vec<SharedString>,
        duration: u64,
    ) {
        cx.emit(NowPlayingEvent::Meta(title, album, artists, duration));
        cx.notify();
    }

    pub fn update_pos(&mut self, cx: &mut Context<Self>, pos: u64) {
        cx.emit(NowPlayingEvent::Position(pos));
        cx.notify();
    }

    pub fn update_thumbnail(&mut self, cx: &mut Context<Self>, thumbnail: Thumbnail) {
        cx.emit(NowPlayingEvent::Thumbnail(thumbnail));
        cx.notify();
    }

    pub fn update_state(&mut self, cx: &mut Context<Self>, state: State) {
        cx.emit(NowPlayingEvent::State(state));
        cx.notify();
    }

    pub fn update_vol(&mut self, cx: &mut Context<Self>, vol: f64) {
        cx.emit(NowPlayingEvent::Volume(vol));
        cx.notify();
    }

    pub fn update_tracks(&mut self, cx: &mut Context<Self>, tracks: Vec<Track>) {
        cx.emit(NowPlayingEvent::Tracks(tracks));
        cx.notify();
    }
}

impl EventEmitter<NowPlayingEvent> for NowPlaying {}
impl Global for NowPlaying {}
