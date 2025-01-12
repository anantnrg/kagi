#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub uri: String,
    pub duration: Option<f64>,
    pub album_art_uri: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Playlist {
    pub name: String,
    pub tracks: Vec<Track>,
    pub current_index: usize,
}
