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
    pub backend: Arc<dyn Backend>,
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
    pub fn new(backend: Arc<dyn Backend>, playlist: Arc<Mutex<Playlist>>) -> (Player, Controller) {
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
                Command::Play => {
                    let playlist = self.playlist.clone();
                    let backend = self.backend.clone();
                    if playlist
                        .lock()
                        .expect("Could not lock playlist")
                        .tracks
                        .len()
                        != 0
                    {
                        if playlist.lock().expect("Could not lock playlist").playing == false {
                            if playlist.lock().expect("Could not lock playlist").loaded == false {
                                playlist
                                    .lock()
                                    .expect("Could not lock playlist")
                                    .load(&backend.clone())
                                    .await
                                    .map_err(|e| self.tx.send(Response::Error(e.to_string())));
                                backend.play().await.expect("Could not play");
                            } else {
                                backend.play().await.expect("Could not play");
                            }
                            playlist.lock().expect("Could not lock playlist").playing = true;
                        }
                    } else {
                        println!("Playlist is not loaded.");
                    }
                }
                Command::Pause => {
                    let playlist = self.playlist.clone();
                    let backend = self.backend.clone();
                    if playlist.lock().expect("Could not lock playlist").playing == true {
                        backend.pause().await.expect("Could not pause playback");
                    }
                }
                Command::GetMeta => {}
                Command::Volume(vol) => {}
            }
        }
    }
}
