use crossbeam_channel::Sender;
use std::path::PathBuf;
use std::time::Duration;

pub struct Controller {
    audio_tx: Sender<AudioCommand>,
    state: PlayerState,
}

pub struct PlayerState {
    pub current: Option<PathBuf>,
    pub playing: bool,
    pub position: Duration,
}

pub enum AudioCommand {
    Load(String),
    Play,
    Pause,
    Volume(f64),
    Seek(u64),
    Stop,
}
