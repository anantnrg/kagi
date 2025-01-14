#[derive(Clone)]
pub struct Layout {
    pub left_sidebar: bool,
    pub central: CentralLayout,
    pub right_sidebar: bool,
}

#[derive(Clone)]
pub enum CentralLayout {
    List,
    Art,
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
