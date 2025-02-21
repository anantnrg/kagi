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

impl Global for Theme {}
