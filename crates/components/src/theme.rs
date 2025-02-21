use gpui::{Global, Rgba, rgb};

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Rgba,
    pub text: Rgba,
    pub icon: Rgba,
    pub background: Rgba,
    pub secondary: Rgba,
    pub border: Rgba,
    pub sidebar_bg: Rgba,
    pub main_bg: Rgba,
    pub titlebar_bg: Rgba,
    pub highlight: Rgba,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            accent: rgb(0xcba6f7),
            text: rgb(0xcdd6f4),
            icon: rgb(0xcdd6f4),
            background: rgb(0x11111B),
            secondary: rgb(0x1e1e2d),
            border: rgb(0x11111B),
            sidebar_bg: rgb(0x11111B),
            main_bg: rgb(0x11111B),
            titlebar_bg: rgb(0x11111B),
            highlight: rgb(0x52cba6f7),
        }
    }
    pub fn new(
        accent: Rgba,
        text: Rgba,
        icon: Rgba,
        background: Rgba,
        secondary: Rgba,
        border: Rgba,
        sidebar_bg: Rgba,
        main_bg: Rgba,
        titlebar_bg: Rgba,
        highlight: Rgba,
    ) -> Self {
        Self {
            accent,
            text,
            icon,
            background,
            secondary,
            sidebar_bg,
            main_bg,
            titlebar_bg,
            highlight,
            border,
        }
    }
}

impl From<backend::theme::Theme> for Theme {
    fn from(theme: backend::theme::Theme) -> Self {
        Self {
            accent: hex_to_rgba(&theme.accent),
            text: hex_to_rgba(&theme.text),
            icon: hex_to_rgba(&theme.icon),
            background: hex_to_rgba(&theme.background),
            secondary: hex_to_rgba(&theme.secondary),
            border: hex_to_rgba(&theme.border),
            sidebar_bg: hex_to_rgba(&theme.sidebar_bg),
            main_bg: hex_to_rgba(&theme.main_bg),
            titlebar_bg: hex_to_rgba(&theme.titlebar_bg),
            highlight: hex_to_rgba(&theme.highlight),
        }
    }
}

impl Into<backend::theme::Theme> for Theme {
    fn into(self) -> backend::theme::Theme {
        backend::theme::Theme {
            accent: rgba_to_hex(self.accent),
            text: rgba_to_hex(self.text),
            icon: rgba_to_hex(self.icon),
            background: rgba_to_hex(self.background),
            secondary: rgba_to_hex(self.secondary),
            border: rgba_to_hex(self.border),
            sidebar_bg: rgba_to_hex(self.sidebar_bg),
            main_bg: rgba_to_hex(self.main_bg),
            titlebar_bg: rgba_to_hex(self.titlebar_bg),
            highlight: rgba_to_hex(self.highlight),
        }
    }
}

impl Global for Theme {}

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
