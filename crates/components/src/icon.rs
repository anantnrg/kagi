use gpui::*;

pub enum Icons {
    Close,
    Maximize,
    Minimize,
    Restore,
}

impl Icons {
    pub fn path(self) -> SharedString {
        match self {
            Self::Close => "",
            Self::Restore => "",
            Self::Maximize => "",
            Self::Minimize => "",
        }
        .into()
    }
}

pub struct Icon {
    pub icon: Icons,
    pub size: Pixels,
}
