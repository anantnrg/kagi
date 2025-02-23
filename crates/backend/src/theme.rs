use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
};

use directories::UserDirs;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Theme {
    pub bg: String,
    pub main: MainTheme,
    pub titlebar: TitlebarTheme,
    pub left_sidebar: LeftSidebarTheme,
    pub right_sidebar: RightSidebarTheme,
    pub control_bar: ControlBarTheme,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TitlebarTheme {
    pub bg: String,
    pub title: String,
    pub icon: String,
    pub hover: String,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct LeftSidebarTheme {
    pub bg: String,
    pub title: String,
    pub item_text: String,
    pub item_border: String,
    pub item_bg: String,
    pub item_hover: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct MainTheme {
    pub bg: String,
    pub title: String,
    pub album: String,
    pub artists: String,
    pub separator: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RightSidebarTheme {
    pub bg: String,
    pub title: String,
    pub item_title: String,
    pub item_artists: String,
    pub item_border: String,
    pub item_bg: String,
    pub item_hover: String,
    pub search_bg: String,
    pub search_text: String,
    pub search_placeholder: String,
    pub search_cursor: String,
    pub search_highlight: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ControlBarTheme {
    pub bg: String,
    pub playbar_bg: String,
    pub playbar_fill: String,
    pub playbar_thumb: String,
    pub text: String,
    pub icons: String,
    pub volume_bg: String,
    pub volume_fill: String,
    pub volume_thumb: String,
    pub hover: String,
}

impl Default for TitlebarTheme {
    fn default() -> Self {
        Self {
            bg: "#11111B".to_string(),
            title: "#cba6f7".to_string(),
            icon: "#cdd6f4".to_string(),
            hover: "#52cba6f7".to_string(),
        }
    }
}

impl Default for LeftSidebarTheme {
    fn default() -> Self {
        Self {
            bg: "#11111B".to_string(),
            title: "#cba6f7".to_string(),
            item_text: "#cdd6f4".to_string(),
            item_border: "#11111B".to_string(),
            item_bg: "#1e1e2d".to_string(),
            item_hover: "#52cba6f7".to_string(),
        }
    }
}

impl Default for MainTheme {
    fn default() -> Self {
        Self {
            bg: "#11111B".to_string(),
            title: "#cba6f7".to_string(),
            album: "#cdd6f4".to_string(),
            artists: "#a6adc8".to_string(),
            separator: "#45475a".to_string(),
        }
    }
}

impl Default for RightSidebarTheme {
    fn default() -> Self {
        Self {
            bg: "#11111B".to_string(),
            title: "#cba6f7".to_string(),
            item_title: "#cdd6f4".to_string(),
            item_artists: "#a6adc8".to_string(),
            item_border: "#11111B".to_string(),
            item_bg: "#1e1e2d".to_string(),
            item_hover: "#52cba6f7".to_string(),
            search_bg: "#181825".to_string(),
            search_text: "#cdd6f4".to_string(),
            search_placeholder: "#a6adc8".to_string(),
            search_cursor: "#cba6f7".to_string(),
            search_highlight: "#52cba6f7".to_string(),
        }
    }
}

impl Default for ControlBarTheme {
    fn default() -> Self {
        Self {
            bg: "#11111B".to_string(),
            playbar_bg: "#181825".to_string(),
            playbar_fill: "#cba6f7".to_string(),
            playbar_thumb: "#cdd6f4".to_string(),
            text: "#cdd6f4".to_string(),
            icons: "#cba6f7".to_string(),
            volume_bg: "#181825".to_string(),
            volume_fill: "#cba6f7".to_string(),
            volume_thumb: "#cdd6f4".to_string(),
            hover: "#cba6f7".to_string(),
        }
    }
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            bg: "#11111b".to_string(),
            main: MainTheme::default(),
            titlebar: TitlebarTheme::default(),
            left_sidebar: LeftSidebarTheme::default(),
            right_sidebar: RightSidebarTheme::default(),
            control_bar: ControlBarTheme::default(),
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
                Self::write(&Theme::default()).expect("Could not write file");
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
