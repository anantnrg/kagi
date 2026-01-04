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
    GetVolume,
    SetVolume,
    Seek,
    Stop,
}
