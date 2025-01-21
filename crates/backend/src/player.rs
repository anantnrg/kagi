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

impl Player {
    pub fn new(
        backend: Arc<Box<dyn Backend>>,
        playlist: Arc<Mutex<Playlist>>,
    ) -> (Player, Controller) {
        let (cmd_tx, cmd_rx) = smol::channel::bounded(128);
        let (res_tx, res_rx) = smol::channel::bounded(128);
        (
            Player {
                backend,
                playlist,
                volume: 0.5,
                tx: res_tx,
                rx: cmd_rx,
            },
            Controller {
                tx: cmd_tx,
                rx: res_rx,
            },
        )
    }

    pub async fn run(&mut self) {
        while let Ok(Command) = self.rx.try_recv() {
            match Command {
                Command::Play => {}
                Command::Pause => {}
                Command::GetMeta => {}
                Command::Volume(vol) => {}
            }
        }
    }
}
