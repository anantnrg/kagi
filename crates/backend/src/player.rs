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

#[derive(Debug, Clone)]
pub enum Response {
    Error(String),
    Success(String),
    Metadata(Track),
}

#[derive(Debug, Clone)]
pub struct Player {
    pub backend: Arc<dyn Backend>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub volume: f64,
    pub tx: Sender<Response>,
    pub rx: Receiver<Command>,
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub tx: Sender<Command>,
    pub rx: Receiver<Response>,
}

impl gpui::Global for Controller {}

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
                                    .map_err(|e| self.tx.send(Response::Error(e.to_string())))
                                    .expect("Could not load track.");
                                backend
                                    .play()
                                    .await
                                    .map_err(|e| self.tx.send(Response::Error(e.to_string())))
                                    .expect("Could not play");
                            } else {
                                backend
                                    .play()
                                    .await
                                    .map_err(|e| self.tx.send(Response::Error(e.to_string())))
                                    .expect("Could not play");
                            }
                            playlist.lock().expect("Could not lock playlist").playing = true;
                            self.tx
                                .send(Response::Success("Playback started.".to_string()))
                                .await
                                .expect("Could not send message");
                        }
                    } else {
                        println!("Playlist is not loaded.");
                        self.tx
                            .send(Response::Error("Playlist is not loaded.".to_string()))
                            .await
                            .expect("Could not send message");
                    }
                }
                Command::Pause => {
                    let playlist = self.playlist.clone();
                    let backend = self.backend.clone();
                    if playlist.lock().expect("Could not lock playlist").playing == true {
                        backend
                            .pause()
                            .await
                            .map_err(|e| self.tx.send(Response::Error(e.to_string())))
                            .expect("Could not pause playback");
                    }
                    self.tx
                        .send(Response::Success("Playback paused.".to_string()))
                        .await
                        .expect("Could not send message");
                }
                Command::GetMeta => {
                    let playlist = self.playlist.clone();

                    if playlist.lock().expect("Could not lock playlist").loaded == true {
                        self.tx
                            .send(Response::Metadata(
                                playlist.lock().expect("Could not lock playlist").tracks[playlist
                                    .lock()
                                    .expect("Could not lock playlist")
                                    .current_index]
                                    .clone(),
                            ))
                            .await
                            .expect("Could not send message");
                    }
                }
                Command::Volume(vol) => {
                    let playlist = self.playlist.clone();
                    let backend = self.backend.clone();

                    if playlist.lock().expect("Could not lock playlist").loaded == true {
                        backend.set_volume(vol).await.expect("Could not set volume");
                        self.tx
                            .send(Response::Success(
                                format!("Volume set to {vol}").to_string(),
                            ))
                            .await
                            .expect("Could not send message");
                    }
                }
            }
        }
    }
}
