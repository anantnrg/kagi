use gpui::*;
use gstreamer::State;

#[derive(Clone)]
pub struct PlayerContext {
    pub metadata: Entity<Metadata>,
    pub state: Entity<PlayerState>,
    pub tracks: Entity<Vec<Track>>,
}

#[derive(Clone)]
pub struct Metadata {
    pub playlist_name: SharedString,
    pub title: SharedString,
    pub album: SharedString,
    pub artists: Vec<SharedString>,
    pub duration: u64,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Clone)]
pub struct PlayerState {
    pub position: u64,
    pub state: State,
    pub volume: f64,
    pub shuffle: bool,
    pub repeat: bool,
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

pub enum PlayerContextEvent {
    Meta(SharedString, SharedString, Vec<SharedString>, u64),
    Position(u64),
    Thumbnail(Thumbnail),
    State(State),
    Volume(f64),
    Tracks(Vec<Track>),
    PlaylistName(String),
    Shuffle(bool),
    Repeat(bool),
}

impl PlayerContext {
    pub fn new() -> Self {
        PlayerContext {
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
            shuffle: false,
            repeat: false,
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
        cx.emit(PlayerContextEvent::Meta(title, album, artists, duration));
        cx.notify();
    }

    pub fn update_pos(&mut self, cx: &mut Context<Self>, pos: u64) {
        cx.emit(PlayerContextEvent::Position(pos));
        cx.notify();
    }

    pub fn update_thumbnail(&mut self, cx: &mut Context<Self>, thumbnail: Thumbnail) {
        cx.emit(PlayerContextEvent::Thumbnail(thumbnail));
        cx.notify();
    }

    pub fn update_state(&mut self, cx: &mut Context<Self>, state: State) {
        cx.emit(PlayerContextEvent::State(state));
        cx.notify();
    }

    pub fn update_vol(&mut self, cx: &mut Context<Self>, vol: f64) {
        cx.emit(PlayerContextEvent::Volume(vol));
        cx.notify();
    }

    pub fn update_tracks(&mut self, cx: &mut Context<Self>, tracks: Vec<Track>) {
        cx.emit(PlayerContextEvent::Tracks(tracks));
        cx.notify();
    }

    pub fn update_playlist_name(&mut self, cx: &mut Context<Self>, name: String) {
        cx.emit(PlayerContextEvent::PlaylistName(name));
        cx.notify();
    }

    pub fn update_shuffle(&mut self, cx: &mut Context<Self>, shuffle: bool) {
        cx.emit(PlayerContextEvent::Shuffle(shuffle));
        cx.notify();
    }

    pub fn update_repeat(&mut self, cx: &mut Context<Self>, repeat: bool) {
        cx.emit(PlayerContextEvent::Repeat(repeat));
        cx.notify();
    }
}

impl EventEmitter<PlayerContextEvent> for PlayerContext {}
impl Global for PlayerContext {}
