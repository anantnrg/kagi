pub struct Layout {
    pub left_sidebar: bool,
    pub central: CentralLayout,
    pub right_sidebar: bool,
}

pub enum CentralLayout {
    List,
    Art,
}
