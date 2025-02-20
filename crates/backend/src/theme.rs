use std::{fs, path::PathBuf};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Theme {
    pub accent: u32,
    pub text: u32,
    pub icon: u32,
    pub background: u32,
    pub secondary: u32,
    pub sidebar_bg: u32,
    pub main_bg: u32,
    pub titlebar_bg: u32,
    pub highlight: u32,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            accent: 0xcba6f7,
            text: 0xcdd6f4,
            icon: 0xcdd6f4,
            background: 0x11111B,
            secondary: 0x1e1e2d,
            sidebar_bg: 0x11111B,
            main_bg: 0x11111B,
            titlebar_bg: 0x11111B,
            highlight: 0x52cba6f7,
        }
    }
    pub fn get_file() -> Option<PathBuf> {
        if let Some(user_dirs) = UserDirs::new() {
            let proj_dir = user_dirs
                .audio_dir()
                .unwrap_or(user_dirs.home_dir())
                .join("Kagi");
            if let Err(e) = fs::create_dir_all(proj_dir.clone()) {
                eprintln!("Could not create config directory: {}", e);
                return None;
            }
            Some(proj_dir.join("theme.toml"))
        } else {
            None
        }
    }
    pub fn load() -> Self {
        if let Some(file_path) = Self::get_file() {
            if file_path.exists() {
                match fs::read_to_string(&file_path) {
                    Ok(contents) => match toml::from_str(&contents) {
                        Ok(saved) => saved,
                        Err(e) => {
                            eprintln!("Failed to parse TOML: {}", e);
                            Theme::default()
                        }
                    },
                    Err(e) => {
                        eprintln!("Failed to read file: {}", e);
                        Theme::default()
                    }
                }
            } else {
                Theme::default()
            }
        } else {
            Theme::default()
        }
    }
}
