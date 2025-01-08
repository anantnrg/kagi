pub struct Layout {
    left_sidebar: bool,
    central: CentralLayout,
    right_sidebar: bool,
}

pub enum CentralLayout {
    List,
    Art,
}
