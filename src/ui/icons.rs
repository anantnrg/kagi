use gpui::*;
use gpui_component::{Icon, IconNamed};

#[derive(IntoElement)]
pub enum Icons {
    Music,
    MusicList,
    WinClose,
    WinMax,
    WinRes,
    WinMin,
    Settings,
}

impl IconNamed for Icons {
    fn path(self) -> gpui::SharedString {
        match self {
            Icons::Music => "icons/music.svg",
            Icons::MusicList => "icons/list-music.svg",
            Icons::WinClose => "icons/window-close.svg",
            Icons::WinMax => "icons/window-maximize.svg",
            Icons::WinRes => "icons/window-restore.svg",
            Icons::WinMin => "icons/window-minimize.svg",
            Icons::Settings => "icons/settings.svg",
        }
        .into()
    }
}

impl RenderOnce for Icons {
    fn render(self, _: &mut Window, _: &mut App) -> impl IntoElement {
        Icon::empty().path(self.path())
    }
}
