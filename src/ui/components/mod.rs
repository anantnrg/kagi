pub mod navbar;
pub mod titlebar;

#[derive(Clone, Copy)]
pub enum Page {
    Home,
    Playlists,
    Settings,
}
