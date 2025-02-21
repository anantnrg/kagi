use std::{
    num::NonZeroUsize,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use gstreamer::State;
use image::{Frame, RgbaImage, imageops::thumbnail};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use rand::seq::SliceRandom;
use ring_channel::{RingReceiver as Receiver, RingSender as Sender};
use serde::{Deserialize, Serialize};
use smallvec::SmallVec;
use std::sync::mpsc;

use crate::{
    Backend,
    playback::{Playlist, SavedPlaylist, SavedPlaylists, Track},
    theme::Theme,
};

pub enum Command {
    Play,
    Pause,
    Volume(f64),
    GetMeta,
    GetTracks,
    Next,
    Previous,
    Seek(u64),
    PlayId(usize),
    LoadFromFolder(SavedPlaylist),
    LoadFolder,
    LoadSavedPlaylists,
    WriteSavedPlaylists,
    RetrieveSavedPlaylists,
    Shuffle,
    LoadTheme,
    WriteTheme(Theme),
}

#[derive(Clone)]
pub enum Response {
    Error(String),
    Warning(String),
    Info(String),
    Metadata(Track),
    StateChanged(State),
    Eos,
    StreamStart,
    Position(u64),
    Thumbnail(Thumbnail),
    Tracks(Vec<Track>),
    SavedPlaylists(SavedPlaylists),
    PlaylistName(String),
    Shuffle(bool),
    Theme(Theme),
}

#[derive(Clone)]
pub struct Player {
    pub backend: Arc<dyn Backend>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub queue: Vec<Track>,
    pub volume: f64,
    pub position: u64,
    pub current_index: usize,
    pub loaded: bool,
    pub playing: bool,
    pub shuffle: bool,
    pub saved_playlists: SavedPlaylists,
    pub tx: Sender<Response>,
    pub rx: Receiver<Command>,
}

#[derive(Debug, Clone)]
pub struct Controller {
    pub tx: Sender<Command>,
    pub rx: Receiver<Response>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Thumbnail {
    pub img: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

impl gpui::Global for Controller {}

impl Player {
    pub fn new(backend: Arc<dyn Backend>, playlist: Arc<Mutex<Playlist>>) -> (Player, Controller) {
        let (cmd_tx, cmd_rx) = ring_channel::ring_channel(NonZeroUsize::new(128).unwrap());
        let (res_tx, res_rx) = ring_channel::ring_channel(NonZeroUsize::new(128).unwrap());
        (
            Player {
                backend,
                playlist,
                queue: vec![],
                volume: 0.5,
                position: 0,
                current_index: 0,
                loaded: false,
                playing: false,
                saved_playlists: SavedPlaylists::default(),
                tx: res_tx,
                rx: cmd_rx,
                shuffle: false,
            },
            Controller {
                tx: cmd_tx,
                rx: res_rx,
            },
        )
    }

    pub fn set_playing(&mut self) {
        self.playing = !self.playing;
    }

    pub async fn load(
        &mut self,
        backend: &Arc<dyn Backend>,
        current_index: usize,
    ) -> anyhow::Result<()> {
        let current_song = &self.queue[current_index];
        backend.load(&current_song.uri).await?;
        Ok(())
    }

    pub async fn play_next(&mut self, backend: &Arc<dyn Backend>) -> anyhow::Result<()> {
        if self.current_index + 1 < self.queue.len() {
            self.current_index += 1;
            {
                self.load(backend, self.current_index).await?;
            }
        }
        Ok(())
    }

    pub async fn play_previous(&mut self, backend: &Arc<dyn Backend>) -> anyhow::Result<()> {
        if self.current_index > 0 {
            self.current_index -= 1;
            {
                self.load(backend, self.current_index).await?;
            }
        }
        Ok(())
    }

    pub async fn play_id(&mut self, backend: &Arc<dyn Backend>, id: usize) -> anyhow::Result<()> {
        self.current_index = id;
        let uri = self.queue[id].uri.clone();
        backend.load(&uri).await?;
        Ok(())
    }

    pub async fn run(&mut self) {
        let theme_file = Theme::get_file().expect("Could not get theme file path.");

        let (watch_tx, watch_rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res: Result<Event, _>| {
            if let Ok(event) = res {
                if let EventKind::Modify(_) = event.kind {
                    watch_tx
                        .send(())
                        .expect("Failed to send theme update event.");
                }
            }
        })
        .expect("Failed to create watcher.");

        watcher
            .watch(&theme_file, RecursiveMode::NonRecursive)
            .expect("Failed to watch theme file.");
        loop {
            while let Ok(command) = self.rx.try_recv() {
                match command {
                    Command::Play => {
                        let backend = self.backend.clone();
                        if !self.queue.is_empty() {
                            if !self.playing {
                                if self.loaded {
                                    let tx = self.tx.clone();
                                    self.tx
                                        .send(Response::StateChanged(State::Playing))
                                        .expect("Could not send message");
                                    let _ = backend
                                        .play()
                                        .await
                                        .map_err(|e| tx.send(Response::Error(e.to_string())));
                                    self.playing = true;
                                } else {
                                    println!("Playlist is not loaded.");
                                    self.tx
                                        .send(Response::Error(
                                            "Playlist is not loaded.".to_string(),
                                        ))
                                        .expect("Could not send message");
                                }
                            }
                        }
                    }
                    Command::Pause => {
                        let backend = self.backend.clone();
                        if self.playing {
                            self.tx
                                .send(Response::StateChanged(State::Paused))
                                .expect("Could not send message");
                            let _ = backend
                                .pause()
                                .await
                                .map_err(|e| self.tx.send(Response::Error(e.to_string())));
                            self.playing = false;
                        }
                    }
                    Command::GetMeta => {
                        if self.loaded {
                            let track = self.queue[self.current_index].clone();
                            self.tx
                                .send(Response::Metadata(track))
                                .expect("Could not send message");
                        }
                    }
                    Command::GetTracks => {
                        if self.loaded {
                            let tracks = self.queue.clone();
                            self.tx
                                .send(Response::Tracks(tracks))
                                .expect("Could not send message");
                        }
                    }
                    Command::Volume(vol) => {
                        let backend = self.backend.clone();
                        if self.loaded {
                            self.tx
                                .send(Response::Info(format!("Volume set to {vol}")))
                                .expect("Could not send message");
                            backend.set_volume(vol).await.expect("Could not set volume");
                            println!("Volume set to {vol}");
                            self.volume = vol;
                        }
                    }
                    Command::Next => {
                        let backend = self.backend.clone();
                        if self.loaded {
                            backend.stop().await.expect("Could not stop");
                            self.play_next(&backend)
                                .await
                                .expect("Could not play next.");
                            self.tx
                                .send(Response::StateChanged(State::Playing))
                                .expect("Could not send message");
                            backend.play().await.expect("Could not play");
                            self.playing = true;
                            backend
                                .set_volume(self.volume)
                                .await
                                .expect("Could not set volume");
                        }
                    }
                    Command::Previous => {
                        let backend = self.backend.clone();
                        if self.loaded {
                            backend.stop().await.expect("Could not stop");
                            self.play_previous(&backend)
                                .await
                                .expect("Could not play previous.");
                            self.tx
                                .send(Response::StateChanged(State::Playing))
                                .expect("Could not send message");
                            backend.play().await.expect("Could not play");
                            self.playing = true;
                            backend
                                .set_volume(self.volume)
                                .await
                                .expect("Could not set volume");
                        }
                    }
                    Command::PlayId(id) => {
                        let backend = self.backend.clone();
                        if self.loaded {
                            backend.stop().await.expect("Could not stop");
                            self.play_id(&backend, id)
                                .await
                                .expect("Could not play track");
                            self.tx
                                .send(Response::StateChanged(State::Playing))
                                .expect("Could not send message");
                            backend.play().await.expect("Could not play");
                            self.playing = true;
                            backend
                                .set_volume(self.volume)
                                .await
                                .expect("Could not set volume");
                        }
                    }
                    Command::LoadFromFolder(saved_playlist) => {
                        let backend = self.backend.clone();
                        let playlist: Playlist;
                        if let Some(cached) =
                            Playlist::read_cached(saved_playlist.cached_name).await
                        {
                            playlist = cached;
                        } else {
                            playlist = Playlist::from_dir(
                                &backend,
                                PathBuf::from(saved_playlist.actual_path),
                            )
                            .await;
                        }

                        self.loaded = true;
                        self.playlist = Arc::new(Mutex::new(playlist.clone()));
                        self.queue = playlist.clone().tracks;

                        self.load(&backend, 0)
                            .await
                            .expect("Could not load first item");
                        self.tx
                            .send(Response::PlaylistName(playlist.name))
                            .expect("Could not send message");
                    }
                    Command::LoadFolder => {
                        let backend = self.backend.clone();
                        if let Some(path) = rfd::AsyncFileDialog::new().pick_folder().await {
                            let path = path.path().to_owned();
                            let name = path
                                .file_name()
                                .and_then(|name| name.to_str())
                                .unwrap_or("unknown playlist")
                                .to_string();
                            let cached_name: String = name
                                .to_lowercase()
                                .chars()
                                .filter_map(|c| {
                                    if c.is_ascii_alphabetic() {
                                        Some(c)
                                    } else if c == ' ' {
                                        Some('_')
                                    } else {
                                        None
                                    }
                                })
                                .collect();
                            let new_saved_playlist = SavedPlaylist {
                                name,
                                actual_path: path.to_string_lossy().to_string(),
                                cached_name: cached_name.clone(),
                            };
                            let playlist =
                                Playlist::from_dir(&backend, PathBuf::from(path.clone())).await;

                            self.loaded = true;
                            self.playlist = Arc::new(Mutex::new(playlist.clone()));
                            self.queue = playlist.clone().tracks;
                            playlist
                                .write_cached(cached_name)
                                .await
                                .expect("Could not write cache");
                            self.tx
                                .send(Response::PlaylistName(playlist.name))
                                .expect("Could not send message");
                            self.load(&backend, 0)
                                .await
                                .expect("Could not load first item");
                            if !self
                                .saved_playlists
                                .playlists
                                .iter()
                                .any(|p| *p == new_saved_playlist)
                            {
                                self.saved_playlists.playlists.push(new_saved_playlist);
                            }
                        }
                    }
                    Command::LoadSavedPlaylists => {
                        self.saved_playlists = SavedPlaylists::load();
                        self.tx
                            .send(Response::SavedPlaylists(self.saved_playlists.clone()))
                            .expect("Could not send message");
                    }
                    Command::RetrieveSavedPlaylists => {
                        self.tx
                            .send(Response::SavedPlaylists(self.saved_playlists.clone()))
                            .expect("Could not send message");
                    }
                    Command::WriteSavedPlaylists => {
                        SavedPlaylists::save_playlists(&self.saved_playlists)
                            .expect("Could not save to file");
                    }
                    Command::Seek(time) => {
                        let backend = self.backend.clone();
                        if self.playing {
                            backend.seek(time).await.expect("Could not seek");
                        }
                    }
                    Command::Shuffle => {
                        let mut rng = rand::rng();
                        if !self.shuffle {
                            self.queue.shuffle(&mut rng);
                            self.shuffle = true;
                        } else {
                            self.queue = self
                                .playlist
                                .lock()
                                .expect("Could not lock playlist")
                                .tracks
                                .clone();
                            self.shuffle = false;
                        }
                        self.tx
                            .send(Response::Tracks(self.queue.clone()))
                            .expect("Could not send message");
                        self.tx
                            .send(Response::Shuffle(self.shuffle.clone()))
                            .expect("Could not send message");
                    }
                    Command::LoadTheme => {
                        let theme = Theme::load();
                        self.tx
                            .send(Response::Theme(theme))
                            .expect("Could not send message");
                    }
                    Command::WriteTheme(theme) => {
                        Theme::write(&theme).expect("Could not write theme");
                    }
                }
            }

            if let Some(res) = self.backend.monitor().await {
                self.tx.send(res).unwrap();
            }

            if watch_rx.try_recv().is_ok() {
                let theme = Theme::load();
                self.tx
                    .send(Response::Theme(theme))
                    .expect("Could not send theme update.");
            }

            let curr_pos = self.backend.get_position().await;
            if self.position != curr_pos {
                self.tx
                    .send(Response::Position(curr_pos))
                    .expect("Could not send message.");
                self.position = curr_pos;
            }
        }
    }
}

impl Controller {
    pub fn load(&self, saved_playlist: SavedPlaylist) {
        self.tx
            .send(Command::LoadFromFolder(saved_playlist))
            .expect("Could not send command");
    }

    pub fn open_folder(&self) {
        self.tx
            .send(Command::LoadFolder)
            .expect("Could not send command");
    }

    pub fn play(&self) {
        self.tx.send(Command::Play).expect("Could not send command");
    }

    pub fn play_id(&self, id: usize) {
        self.tx
            .send(Command::PlayId(id))
            .expect("Could not send command");
    }

    pub fn pause(&self) {
        self.tx
            .send(Command::Pause)
            .expect("Could not send command");
    }

    pub fn next(&self) {
        self.tx.send(Command::Next).expect("Could not send command");
    }

    pub fn prev(&self) {
        self.tx
            .send(Command::Previous)
            .expect("Could not send command");
    }

    pub fn get_meta(&self) {
        self.tx
            .send(Command::GetMeta)
            .expect("Could not send command");
    }

    pub fn get_queue(&self) {
        self.tx
            .send(Command::GetTracks)
            .expect("Could not send command");
    }

    pub fn volume(&self, vol: f64) {
        self.tx
            .send(Command::Volume(vol))
            .expect("Could not send command");
    }

    pub fn load_saved_playlists(&self) {
        self.tx
            .send(Command::LoadSavedPlaylists)
            .expect("Could not send command");
    }

    pub fn write_playlist(&self) {
        self.tx
            .send(Command::WriteSavedPlaylists)
            .expect("Could not send command");
    }

    pub fn retrieve_saved_playlists(&self) {
        self.tx
            .send(Command::RetrieveSavedPlaylists)
            .expect("Could not send command");
    }

    pub fn seek(&self, time: u64) {
        self.tx
            .send(Command::Seek(time))
            .expect("Could not send command");
    }

    pub fn shuffle(&self) {
        self.tx
            .send(Command::Shuffle)
            .expect("Could not send command");
    }

    pub fn load_theme(&self) {
        self.tx
            .send(Command::LoadTheme)
            .expect("Could not send command");
    }

    pub fn write_theme(&self, theme: Theme) {
        self.tx
            .send(Command::WriteTheme(theme))
            .expect("Could not send command");
    }
}

impl Thumbnail {
    pub fn to_frame(&self) -> SmallVec<[Frame; 1]> {
        let img = RgbaImage::from_raw(self.width, self.height, self.img.clone())
            .expect("Failed to reconstruct image from raw bytes");
        SmallVec::from_vec(vec![Frame::new(thumbnail(&img, self.width, self.height))])
    }
}
