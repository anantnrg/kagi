use image::Frame;
use smallvec::SmallVec;
use std::{path::PathBuf, sync::Arc};

use crate::Backend;

#[derive(Clone)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub uri: String,
    pub duration: u64,
    pub album_art_uri: Option<SmallVec<[Frame; 1]>>,
}

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub current_index: usize,
    pub loaded: bool,
    pub playing: bool,
}

impl Track {
    pub fn default() -> Self {
        Track {
            album: "Unknown Album".into(),
            album_art_uri: None,
            artists: vec!["Unknown Artist".into()],
            duration: 0,
            title: "Unknown Track".into(),
            uri: "".to_string(),
        }
    }
}

impl Playlist {
    pub fn default() -> Self {
        Playlist {
            name: "Unknown Playlist".to_string(),
            tracks: vec![],
            current_index: 0,
            loaded: false,
            playing: false,
        }
    }
    pub async fn from_dir(backend: &Arc<dyn Backend>, dir: PathBuf) -> Self {
        let mut playlist = Playlist {
            name: dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown Playlist".into()),
            tracks: Vec::new(),
            current_index: 0,
            loaded: false,
            playing: false,
        };

        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();

                    if let Some(ext) = path.extension() {
                        let ext = ext.to_string_lossy().to_lowercase();
                        if ext == "mp3" || ext == "flac" || ext == "wav" || ext == "ogg" {
                            let uri =
                                format!("file:///{}", path.to_string_lossy().replace("\\", "/"));

                            let track = match backend.get_meta(&uri).await {
                                Ok(t) => t,
                                Err(_) => {
                                    eprintln!("Failed to load metadata for {:?}", uri);
                                    Track {
                                        title: path
                                            .file_stem()
                                            .map(|n| n.to_string_lossy().to_string())
                                            .unwrap_or_else(|| "Unknown Track".into()),
                                        uri: uri.clone(),
                                        ..Track::default()
                                    }
                                }
                            };
                            playlist.tracks.push(track);
                        }
                    }
                }
            }
        }
        playlist.loaded = true;
        playlist
    }

    pub fn set_playing(&mut self) {
        self.playing = !self.playing;
    }

    pub async fn load(&mut self, backend: &Arc<dyn Backend>) -> anyhow::Result<()> {
        let current_song = &self.tracks[self.current_index];
        backend.load(&current_song.uri).await?;
        Ok(())
    }

    pub async fn play_next(&mut self, backend: &Arc<dyn Backend>) -> anyhow::Result<()> {
        if self.current_index + 1 < self.tracks.len() {
            self.current_index += 1;
            self.load(backend).await?;
        }
        Ok(())
    }

    pub async fn play_previous(&mut self, backend: &Arc<dyn Backend>) -> anyhow::Result<()> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.load(backend).await?;
        }
        Ok(())
    }
}
