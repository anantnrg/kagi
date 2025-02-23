use gpui::{Global, Rgba, rgb};

#[derive(Clone, Copy)]
pub struct Theme {
    pub bg: Rgba,
    pub main: MainTheme,
    pub titlebar: TitlebarTheme,
    pub left_sidebar: LeftSidebarTheme,
    pub right_sidebar: RightSidebarTheme,
    pub control_bar: ControlBarTheme,
}

impl Global for Theme {}

#[derive(Clone, Copy)]
pub struct TitlebarTheme {
    pub bg: Rgba,
    pub title: Rgba,
    pub icon: Rgba,
    pub hover: Rgba,
}

#[derive(Clone, Copy)]
pub struct LeftSidebarTheme {
    pub bg: Rgba,
    pub title: Rgba,
    pub item_text: Rgba,
    pub item_border: Rgba,
    pub item_bg: Rgba,
    pub item_hover: Rgba,
}

#[derive(Clone, Copy)]
pub struct MainTheme {
    pub bg: Rgba,
    pub title: Rgba,
    pub album: Rgba,
    pub artists: Rgba,
    pub separator: Rgba,
}

#[derive(Clone, Copy)]
pub struct RightSidebarTheme {
    pub bg: Rgba,
    pub title: Rgba,
    pub item_title: Rgba,
    pub item_artists: Rgba,
    pub item_border: Rgba,
    pub item_bg: Rgba,
    pub item_hover: Rgba,
    pub search_bg: Rgba,
    pub search_text: Rgba,
    pub search_placeholder: Rgba,
    pub search_cursor: Rgba,
    pub search_highlight: Rgba,
}

#[derive(Clone, Copy)]
pub struct ControlBarTheme {
    pub bg: Rgba,
    pub playbar_bg: Rgba,
    pub playbar_fill: Rgba,
    pub playbar_thumb: Rgba,
    pub text: Rgba,
    pub icons: Rgba,
    pub volume_bg: Rgba,
    pub volume_fill: Rgba,
    pub volume_thumb: Rgba,
    pub hover: Rgba,
}

impl Default for TitlebarTheme {
    fn default() -> Self {
        Self {
            bg: rgb(0x11111B),
            title: rgb(0xcba6f7),
            icon: rgb(0xcdd6f4),
            hover: rgb(0x52cba6f7),
        }
    }
}

impl Default for LeftSidebarTheme {
    fn default() -> Self {
        Self {
            bg: rgb(0x11111B),
            title: rgb(0xcba6f7),
            item_text: rgb(0xcdd6f4),
            item_border: rgb(0x1e1e2d),
            item_bg: rgb(0x1e1e2d),
            item_hover: rgb(0x52cba6f7),
        }
    }
}

impl Default for MainTheme {
    fn default() -> Self {
        Self {
            bg: rgb(0x11111B),
            title: rgb(0xcba6f7),
            album: rgb(0xcdd6f4),
            artists: rgb(0xb4befe),
            separator: rgb(0x45475a),
        }
    }
}

impl Default for RightSidebarTheme {
    fn default() -> Self {
        Self {
            bg: rgb(0x11111B),
            title: rgb(0xcba6f7),
            item_title: rgb(0xcdd6f4),
            item_artists: rgb(0xb4befe),
            item_border: rgb(0x1e1e2d),
            item_bg: rgb(0x1e1e2d),
            item_hover: rgb(0x52cba6f7),
            search_bg: rgb(0x1e1e2d),
            search_text: rgb(0xcdd6f4),
            search_placeholder: rgb(0x45475a),
            search_cursor: rgb(0xcba6f7),
            search_highlight: rgb(0x52cba6f7),
        }
    }
}

impl Default for ControlBarTheme {
    fn default() -> Self {
        Self {
            bg: rgb(0x11111B),
            playbar_bg: rgb(0x1e1e2d),
            playbar_fill: rgb(0xcba6f7),
            playbar_thumb: rgb(0x52cba6f7),
            text: rgb(0xcdd6f4),
            icons: rgb(0xcdd6f4),
            volume_bg: rgb(0x1e1e2d),
            volume_fill: rgb(0xcba6f7),
            volume_thumb: rgb(0xcba6f7),
            hover: rgb(0xcba6f7),
        }
    }
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            bg: rgb(0x11111b),
            main: MainTheme::default(),
            titlebar: TitlebarTheme::default(),
            left_sidebar: LeftSidebarTheme::default(),
            right_sidebar: RightSidebarTheme::default(),
            control_bar: ControlBarTheme::default(),
        }
    }
}

impl From<backend::theme::Theme> for Theme {
    fn from(value: backend::theme::Theme) -> Self {
        Theme {
            bg: hex_to_rgba(value.bg.as_str()),
            main: value.main.into(),
            titlebar: value.titlebar.into(),
            left_sidebar: value.left_sidebar.into(),
            right_sidebar: value.right_sidebar.into(),
            control_bar: value.control_bar.into(),
        }
    }
}

impl Into<backend::theme::Theme> for Theme {
    fn into(self) -> backend::theme::Theme {
        backend::theme::Theme {
            bg: rgba_to_hex(self.bg),
            main: self.main.into(),
            titlebar: self.titlebar.into(),
            left_sidebar: self.left_sidebar.into(),
            right_sidebar: self.right_sidebar.into(),
            control_bar: self.control_bar.into(),
        }
    }
}

impl From<backend::theme::TitlebarTheme> for TitlebarTheme {
    fn from(theme: backend::theme::TitlebarTheme) -> Self {
        Self {
            bg: hex_to_rgba(&theme.bg),
            title: hex_to_rgba(&theme.title),
            icon: hex_to_rgba(&theme.icon),
            hover: hex_to_rgba(&theme.hover),
        }
    }
}

impl Into<backend::theme::TitlebarTheme> for TitlebarTheme {
    fn into(self) -> backend::theme::TitlebarTheme {
        backend::theme::TitlebarTheme {
            bg: rgba_to_hex(self.bg),
            title: rgba_to_hex(self.title),
            icon: rgba_to_hex(self.icon),
            hover: rgba_to_hex(self.hover),
        }
    }
}

impl From<backend::theme::LeftSidebarTheme> for LeftSidebarTheme {
    fn from(theme: backend::theme::LeftSidebarTheme) -> Self {
        Self {
            bg: hex_to_rgba(&theme.bg),
            title: hex_to_rgba(&theme.title),
            item_text: hex_to_rgba(&theme.item_text),
            item_border: hex_to_rgba(&theme.item_border),
            item_bg: hex_to_rgba(&theme.item_bg),
            item_hover: hex_to_rgba(&theme.item_hover),
        }
    }
}

impl Into<backend::theme::LeftSidebarTheme> for LeftSidebarTheme {
    fn into(self) -> backend::theme::LeftSidebarTheme {
        backend::theme::LeftSidebarTheme {
            bg: rgba_to_hex(self.bg),
            title: rgba_to_hex(self.title),
            item_text: rgba_to_hex(self.item_text),
            item_border: rgba_to_hex(self.item_border),
            item_bg: rgba_to_hex(self.item_bg),
            item_hover: rgba_to_hex(self.item_hover),
        }
    }
}

impl From<backend::theme::MainTheme> for MainTheme {
    fn from(theme: backend::theme::MainTheme) -> Self {
        Self {
            bg: hex_to_rgba(&theme.bg),
            title: hex_to_rgba(&theme.title),
            album: hex_to_rgba(&theme.album),
            artists: hex_to_rgba(&theme.artists),
            separator: hex_to_rgba(&theme.separator),
        }
    }
}

impl Into<backend::theme::MainTheme> for MainTheme {
    fn into(self) -> backend::theme::MainTheme {
        backend::theme::MainTheme {
            bg: rgba_to_hex(self.bg),
            title: rgba_to_hex(self.title),
            album: rgba_to_hex(self.album),
            artists: rgba_to_hex(self.artists),
            separator: rgba_to_hex(self.separator),
        }
    }
}

impl From<backend::theme::RightSidebarTheme> for RightSidebarTheme {
    fn from(theme: backend::theme::RightSidebarTheme) -> Self {
        Self {
            bg: hex_to_rgba(&theme.bg),
            title: hex_to_rgba(&theme.title),
            item_title: hex_to_rgba(&theme.item_title),
            item_artists: hex_to_rgba(&theme.item_artists),
            item_border: hex_to_rgba(&theme.item_border),
            item_bg: hex_to_rgba(&theme.item_bg),
            item_hover: hex_to_rgba(&theme.item_hover),
            search_bg: hex_to_rgba(&theme.search_bg),
            search_text: hex_to_rgba(&theme.search_text),
            search_placeholder: hex_to_rgba(&theme.search_placeholder),
            search_cursor: hex_to_rgba(&theme.search_cursor),
            search_highlight: hex_to_rgba(&theme.search_highlight),
        }
    }
}

impl Into<backend::theme::RightSidebarTheme> for RightSidebarTheme {
    fn into(self) -> backend::theme::RightSidebarTheme {
        backend::theme::RightSidebarTheme {
            bg: rgba_to_hex(self.bg),
            title: rgba_to_hex(self.title),
            item_title: rgba_to_hex(self.item_title),
            item_artists: rgba_to_hex(self.item_artists),
            item_border: rgba_to_hex(self.item_border),
            item_bg: rgba_to_hex(self.item_bg),
            item_hover: rgba_to_hex(self.item_hover),
            search_bg: rgba_to_hex(self.search_bg),
            search_text: rgba_to_hex(self.search_text),
            search_placeholder: rgba_to_hex(self.search_placeholder),
            search_cursor: rgba_to_hex(self.search_cursor),
            search_highlight: rgba_to_hex(self.search_highlight),
        }
    }
}

impl From<backend::theme::ControlBarTheme> for ControlBarTheme {
    fn from(theme: backend::theme::ControlBarTheme) -> Self {
        Self {
            bg: hex_to_rgba(&theme.bg),
            playbar_bg: hex_to_rgba(&theme.playbar_bg),
            playbar_fill: hex_to_rgba(&theme.playbar_fill),
            playbar_thumb: hex_to_rgba(&theme.playbar_thumb),
            text: hex_to_rgba(&theme.text),
            icons: hex_to_rgba(&theme.icons),
            volume_bg: hex_to_rgba(&theme.volume_bg),
            volume_fill: hex_to_rgba(&theme.volume_fill),
            volume_thumb: hex_to_rgba(&theme.volume_thumb),
            hover: hex_to_rgba(&theme.hover),
        }
    }
}

impl Into<backend::theme::ControlBarTheme> for ControlBarTheme {
    fn into(self) -> backend::theme::ControlBarTheme {
        backend::theme::ControlBarTheme {
            bg: rgba_to_hex(self.bg),
            playbar_bg: rgba_to_hex(self.playbar_bg),
            playbar_fill: rgba_to_hex(self.playbar_fill),
            playbar_thumb: rgba_to_hex(self.playbar_thumb),
            text: rgba_to_hex(self.text),
            icons: rgba_to_hex(self.icons),
            volume_bg: rgba_to_hex(self.volume_bg),
            volume_fill: rgba_to_hex(self.volume_fill),
            volume_thumb: rgba_to_hex(self.volume_thumb),
            hover: rgba_to_hex(self.hover),
        }
    }
}

pub fn rgba_to_hex(color: Rgba) -> String {
    let a = (color.a * 255.0) as u32;
    let r = (color.r * 255.0) as u32;
    let g = (color.g * 255.0) as u32;
    let b = (color.b * 255.0) as u32;
    format!("#{:02X}{:02X}{:02X}{:02X}", a, r, g, b)
}

pub fn hex_to_rgba(hex: &str) -> Rgba {
    let hex = hex.trim_start_matches('#');

    let (a, r, g, b) = match hex.len() {
        6 => (
            255,
            u8::from_str_radix(&hex[0..2], 16).unwrap(),
            u8::from_str_radix(&hex[2..4], 16).unwrap(),
            u8::from_str_radix(&hex[4..6], 16).unwrap(),
        ),
        8 => (
            u8::from_str_radix(&hex[0..2], 16).unwrap(),
            u8::from_str_radix(&hex[2..4], 16).unwrap(),
            u8::from_str_radix(&hex[4..6], 16).unwrap(),
            u8::from_str_radix(&hex[6..8], 16).unwrap(),
        ),
        _ => panic!("Invalid hex color: {}", hex),
    };

    Rgba {
        a: a as f32 / 255.0,
        r: r as f32 / 255.0,
        g: g as f32 / 255.0,
        b: b as f32 / 255.0,
    }
}
