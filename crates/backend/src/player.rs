use std::sync::{Arc, Mutex};

use smol::channel::{Receiver, Sender};

use crate::{Backend, playback::Playlist};

pub struct Player {
    pub backend: Arc<Box<dyn Backend>>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub volume: f64,
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}
