use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Theme {
    pub accent: String,
    pub text: String,
    pub icon: String,
    pub background: String,
    pub secondary: String,
    pub sidebar_bg: String,
    pub main_bg: String,
    pub titlebar_bg: String,
    pub highlight: String,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            accent: String::from("#cba6f7"),
            text: String::from("#cdd6f4"),
            icon: String::from("#cdd6f4"),
            background: String::from("#11111B"),
            secondary: String::from("#1e1e2d"),
            sidebar_bg: String::from("#11111B"),
            main_bg: String::from("#11111B"),
            titlebar_bg: String::from("#11111B"),
            highlight: String::from("#52cba6f7"),
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

    pub fn write(new: &Theme) -> io::Result<()> {
        if let Some(file_path) = Self::get_file() {
            let toml_str = toml::to_string_pretty(new).expect("Failed to serialize Theme");
            let mut file = fs::File::create(file_path)?;
            file.write_all(toml_str.as_bytes())?;
        }
        Ok(())
    }
}
