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
        }
        .into()
    }
}

#[derive(IntoElement)]
pub struct Icon {
    pub icon: Icons,
    pub size: f32,
    pub color: u32,
}

impl Icon {
    pub fn new(icon: Icons) -> Self {
        Icon {
            icon,
            size: 24.0,
            color: 0xcdd6f4,
        }
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn color(mut self, color: u32) -> Self {
        self.color = color;
        self
    }
}

impl RenderOnce for Icon {
    fn render(self, _win: &mut Window, _cx: &mut App) -> impl IntoElement {
        svg()
            .size(px(self.size))
            .text_color(rgb(self.color))
            .path(self.icon.path())
    }
}
