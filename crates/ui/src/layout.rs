#[derive(Clone)]
pub struct Layout {
    pub left_sidebar: LeftSidebarLayout,
    pub central: CentralLayout,
    pub right_sidebar: RightSidebarLayout,
}

#[derive(Clone)]
pub struct LeftSidebarLayout {
    pub show: bool,
    pub width: f32,
}

#[derive(Clone)]
pub struct RightSidebarLayout {
    pub show: bool,
    pub width: f32,
}

#[derive(Clone)]
pub enum CentralLayout {
    List,
    Art,
}

impl LeftSidebarLayout {
    pub fn new() -> Self {
        LeftSidebarLayout {
            show: false,
            width: 0.0,
        }
    }
}

impl RightSidebarLayout {
    pub fn new() -> Self {
        RightSidebarLayout {
            show: false,
            width: 0.0,
        }
    }
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            left_sidebar: true,
            central: CentralLayout::List,
            right_sidebar: true,
        }
    }
}
