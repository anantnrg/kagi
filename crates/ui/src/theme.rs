use gpui::{Hsla, rgb};

pub struct Theme {
    pub accent: Hsla,
    pub text: Hsla,
    pub background: Hsla,
    pub secondary: Hsla,
    pub sidebar_bg: Hsla,
    pub main_bg: Hsla,
    pub titlebar_bg: Hsla,
}

impl Theme {
    pub fn default() -> Self {
        Theme {
            accent: rgb(0x8265BB),
            text: rgb(0xcdd6f4),
            background: rgb(0x181818),
            secondary: rgb(0x45475a),
            sidebar_bg: rgb(0x181818),
            main_bg: rgb(0x181818),
            titlebar_bg: rgb(0x181818),
        }
    }
    pub fn new(
        accent: Hsla,
        text: Hsla,
        background: Hsla,
        secondary: Hsla,
        sidebar_bg: Hsla,
        main_bg: Hsla,
        titlebar_bg: Hsla,
    ) -> Self {
        Self {
            accent,
            text,
            background,
            secondary,
            sidebar_bg,
            main_bg,
            titlebar_bg,
        }
    }
}
