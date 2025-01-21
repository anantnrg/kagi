use std::sync::{Arc, Mutex};

use smol::channel::{Receiver, Sender};

use crate::{
    Backend,
    playback::{Playlist, Track},
};

pub enum Command {
    Play,
    Pause,
    Volume(f64),
    GetMeta,
}

pub enum Response {
    Error(String),
    Success(String),
    Metadata(Track),
}

pub struct Player {
    pub backend: Arc<Box<dyn Backend>>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub volume: f64,
    pub tx: Sender<Response>,
    pub rx: Receiver<Command>,
}

pub struct Controller {
    pub tx: Sender<Command>,
    pub rx: Receiver<Response>,
}
