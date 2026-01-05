use crate::audio::engine::PlaybackState;
use crossbeam_channel::Sender;
use std::path::PathBuf;
use std::time::Duration;

pub struct Controller {
    audio_tx: Sender<AudioCommand>,
    state: PlayerState,
}

pub struct PlayerState {
    pub current: Option<PathBuf>,
    pub state: PlaybackState,
    pub position: u64,
}

pub enum AudioCommand {
    Load(String),
    Play,
    Pause,
    Volume(f32),
    Seek(u64),
    Stop,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AudioEvent {
    Position(u64),
    StateChanged(PlaybackState),
    TrackEnded,
}
