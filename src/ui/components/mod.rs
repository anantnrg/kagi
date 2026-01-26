pub mod navbar;
pub mod titlebar;

#[derive(Clone, Copy, PartialEq)]
pub enum Page {
    Home,
    Playlists,
    Settings,
}

impl gpui::Global for Page {}
