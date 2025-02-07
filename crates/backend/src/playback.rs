use std::{path::PathBuf, sync::Arc};

use serde::{Deserialize, Serialize};

use crate::{Backend, player::Thumbnail};

#[derive(Clone)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub uri: String,
    pub duration: u64,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Clone)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPlaylists {
    pub playlist: Vec<SavedPlaylist>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPlaylist {
    pub name: String,
    pub actual_path: String,
    pub cached_name: String,
}

impl Track {
    pub fn default() -> Self {
        Track {
            album: "Unknown Album".into(),
            artists: vec!["Unknown Artist".into()],
            duration: 0,
            title: "Unknown Track".into(),
            uri: "".to_string(),
            thumbnail: None,
        }
    }
}

impl Playlist {
    pub fn default() -> Self {
        Playlist {
            name: "Unknown Playlist".to_string(),
            tracks: vec![],
        }
    }
    pub async fn from_dir(backend: &Arc<dyn Backend>, dir: PathBuf) -> Self {
        let mut playlist = Playlist {
            name: dir
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "Unknown Playlist".into()),
            tracks: Vec::new(),
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
        playlist
    }

    pub async fn load(
        &mut self,
        backend: &Arc<dyn Backend>,
        current_index: usize,
    ) -> anyhow::Result<()> {
        let current_song = &self.tracks[current_index];
        backend.load(&current_song.uri).await?;
        Ok(())
    }
}
