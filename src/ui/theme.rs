use gpui::{Rgba, rgba};

pub struct Theme {
    pub bg: Rgba,
    pub panel: Rgba,
    pub highlighted: Rgba,

    pub accent: Rgba,

    pub text: Rgba,
    pub text_muted: Rgba,

    pub border: Rgba,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            bg: rgba(0x131319),
            panel: rgba(0x14141d),
            highlighted: rgba(0x202332),
            accent: rgba(0x7351fe),
            text: rgba(0xceced2),
            text_muted: rgba(0x5e6067),
            border: rgba(0x1b1d25),
        }
    }
}
