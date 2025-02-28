use gpui::*;

pub enum Icons {
    Close,
    Maximize,
    Minimize,
    Restore,
    LeftSidebar,
    RightSidebar,
    Play,
    Pause,
    Next,
    Previous,
    Stopped,
    Shuffle,
    Repeat,
    Search,
    Playlist,
    Library,
    FastForward,
    Rewind,
    VolZero,
    VolLow,
    VolMid,
    VolHigh,
}

impl Icons {
    pub fn path(self) -> SharedString {
        match self {
            Self::Close => "icons/close.svg",
            Self::Maximize => "icons/maximize.svg",
            Self::Minimize => "icons/minimize.svg",
            Self::Restore => "icons/restore.svg",
            Self::LeftSidebar => "icons/left_sidebar.svg",
            Self::RightSidebar => "icons/right_sidebar.svg",
            Self::Play => "icons/play.svg",
            Self::Pause => "icons/pause.svg",
            Self::Next => "icons/next.svg",
            Self::Previous => "icons/previous.svg",
            Self::Stopped => "icons/stopped.svg",
            Self::Shuffle => "icons/shuffle.svg",
            Self::Repeat => "icons/repeat.svg",
            Self::Search => "icons/search.svg",
            Self::Library => "icons/library.svg",
            Self::Playlist => "icons/playlist.svg",
            Self::FastForward => "icons/fast-forward.svg",
            Self::Rewind => "icons/rewind.svg",
            Self::VolZero => "icons/vol_zero.svg",
            Self::VolLow => "icons/vol_low.svg",
            Self::VolMid => "icons/vol_mid.svg",
            Self::VolHigh => "icons/vol_high.svg",
        }
        .into()
    }
}

#[derive(IntoElement)]
pub struct Icon {
    pub icon: Icons,
    pub size: f32,
    pub color: Rgba,
    pub hover: Rgba,
}

impl Icon {
    pub fn new(icon: Icons) -> Self {
        Icon {
            icon,
            size: 24.0,
            color: rgb(0xcdd6f4),
            hover: rgb(0xcdd6f4),
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: Rgba) -> Self {
        self.color = color;
        self
    }

    pub fn hover(mut self, color: Rgba) -> Self {
        self.hover = color;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _win: &mut Window, cx: &mut App) -> impl IntoElement {
        svg()
            .flex()
            .items_center()
            .justify_center()
            .size(px(self.size))
            .text_color(self.color)
            .path(self.icon.path())
    }
}
