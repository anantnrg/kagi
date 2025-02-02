#[derive(Clone)]
pub struct Layout {
    pub left_sidebar: SidebarLayout,
    pub central: CentralLayout,
    pub right_sidebar: SidebarLayout,
}

#[derive(Clone)]
pub struct SidebarLayout {
    pub show: bool,
    pub width: f32,
    pub should_show: bool,
}

#[derive(Clone)]
pub enum CentralLayout {
    List,
    Art,
}

impl SidebarLayout {
    pub fn new() -> Self {
        SidebarLayout {
            show: false,
            width: 0.0,
            should_show: false,
        }
    }
}

impl Layout {
    pub fn new() -> Layout {
        Layout {
            left_sidebar: SidebarLayout::new(),
            central: CentralLayout::List,
            right_sidebar: SidebarLayout::new(),
        }
    }

    pub fn get_left_sidebar(&mut self) -> SidebarLayout {
        self.left_sidebar.clone()
    }

    pub fn get_right_sidebar(&mut self) -> SidebarLayout {
        self.right_sidebar.clone()
    }
}
