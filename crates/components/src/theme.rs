use gpui::{Rgba, rgb};

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
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            accent: rgb(0xcba6f7),
            text: rgb(0xcdd6f4),
            icon: rgb(0xcdd6f4),
            background: rgb(0x11111B),
            secondary: rgb(0x45475a),
            sidebar_bg: rgb(0x11111B),
            main_bg: rgb(0x11111B),
            titlebar_bg: rgb(0x11111B),
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
        }
    }
}
