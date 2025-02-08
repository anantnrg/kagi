use std::{
    fs::{self, File},
    io::{self, Write},
    path::PathBuf,
    sync::Arc,
};

use bincode::config;
use directories::UserDirs;
use serde::{Deserialize, Serialize};

use crate::{Backend, player::Thumbnail};

#[derive(Clone, Serialize, Deserialize)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
    pub album: String,
    pub uri: String,
    pub duration: u64,
    pub thumbnail: Option<Thumbnail>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedPlaylists {
    pub playlists: Vec<SavedPlaylist>,
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
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

    pub async fn write_cached(&self, cached_name: String) -> anyhow::Result<()> {
        let cached_path = UserDirs::new()
            .unwrap()
            .audio_dir()
            .unwrap_or(UserDirs::new().unwrap().home_dir())
            .join("Reyvr")
            .join("cache")
            .join(cached_name);
        println!("cached: {:#?}", cached_path.clone());

        let mut cached_file = File::create(cached_path)?;
        let serialized = &bincode::serde::encode_to_vec(self, config::standard())?;
        cached_file.write(serialized)?;

        Ok(())
    }

    pub async fn read_cached(cached_name: String) -> Option<Playlist> {
        let cached_path = UserDirs::new()
            .unwrap()
            .audio_dir()
            .unwrap_or(UserDirs::new().unwrap().home_dir())
            .join("Reyvr")
            .join("cache")
            .join(cached_name);
        println!("cached: {:#?}", cached_path.clone());

        if cached_path.exists() {
            let cached_data = &fs::read(cached_path).expect("Could not read file");
            let deserialized: Playlist =
                bincode::serde::decode_from_slice(cached_data, config::standard())
                    .expect("Could not decode playlist")
                    .0;
            return Some(deserialized);
        } else {
            return None;
        }
    }
}

impl SavedPlaylists {
    pub fn default() -> Self {
        SavedPlaylists { playlists: vec![] }
    }
    pub fn get_playlists_file() -> Option<PathBuf> {
        if let Some(user_dirs) = UserDirs::new() {
            let proj_dir = user_dirs
                .audio_dir()
                .unwrap_or(user_dirs.home_dir())
                .join("Reyvr");
            if let Err(e) = fs::create_dir_all(proj_dir.clone()) {
                eprintln!("Could not create config directory: {}", e);
                return None;
            }
            Some(proj_dir.join("playlists.toml"))
        } else {
            None
        }
    }
    pub fn load() -> Self {
        if let Some(file_path) = Self::get_playlists_file() {
            if file_path.exists() {
                match fs::read_to_string(&file_path) {
                    Ok(contents) => match toml::from_str(&contents) {
                        Ok(saved) => saved,
                        Err(e) => {
                            eprintln!("Failed to parse TOML: {}", e);
                            SavedPlaylists::default()
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                        SavedPlaylists::default()
                    }
                }
            } else {
                SavedPlaylists::default()
            }
        } else {
            SavedPlaylists::default()
        }
    }
    pub fn save_playlists(saved: &SavedPlaylists) -> io::Result<()> {
        if let Some(file_path) = Self::get_playlists_file() {
            let toml_str =
                toml::to_string_pretty(saved).expect("Failed to serialize SavedPlaylists");
            let mut file = fs::File::create(file_path)?;
            file.write_all(toml_str.as_bytes())?;
        }
        Ok(())
    }
}
