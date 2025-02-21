use gpui::{Global, Rgba, rgb};

#[derive(Clone, Copy)]
pub struct Theme {
    pub accent: Rgba,
    pub text: Rgba,
    pub icon: Rgba,
    pub background: Rgba,
    pub secondary: Rgba,
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
        }
    }
}

impl From<backend::theme::Theme> for Theme {
    fn from(theme: backend::theme::Theme) -> Self {
        Self {
            accent: rgb(theme.accent),
            text: rgb(theme.text),
            icon: rgb(theme.icon),
            background: rgb(theme.background),
            secondary: rgb(theme.secondary),
            sidebar_bg: rgb(theme.sidebar_bg),
            main_bg: rgb(theme.main_bg),
            titlebar_bg: rgb(theme.titlebar_bg),
            highlight: rgb(theme.highlight),
        }
    }
}

impl Into<backend::theme::Theme> for Theme {
    fn into(self) -> backend::theme::Theme {
        backend::theme::Theme {
            accent: rgba_to_u32(self.accent),
            text: rgba_to_u32(self.text),
            icon: rgba_to_u32(self.icon),
            background: rgba_to_u32(self.background),
            secondary: rgba_to_u32(self.secondary),
            sidebar_bg: rgba_to_u32(self.sidebar_bg),
            main_bg: rgba_to_u32(self.main_bg),
            titlebar_bg: rgba_to_u32(self.titlebar_bg),
            highlight: rgba_to_u32(self.highlight),
        }
    }
}

impl Global for Theme {}

pub fn rgba_to_u32(color: Rgba) -> u32 {
    let a = (color.a * 255.0) as u32;
    let r = (color.r * 255.0) as u32;
    let g = (color.g * 255.0) as u32;
    let b = (color.b * 255.0) as u32;
    (a << 24) | (r << 16) | (g << 8) | b
}
