use gpui::{rgb, Rgba};

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
            bg: rgb(0x131319),
            panel: rgb(0x14141d),
            highlighted: rgb(0x202332),
            accent: rgb(0x7351fe),
            text: rgb(0xceced2),
            text_muted: rgb(0x5e6067),
            border: rgb(0x1b1d25),
        }
    }
}

impl gpui::Global for Theme {}
