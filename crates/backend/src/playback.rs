use crate::Backend;

#[derive(Debug, Clone)]
pub struct Track {
    pub title: String,
    pub artists: Vec<String>,
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
    pub loaded: bool,
}

impl Playlist {
    // pub fn new<B: Backend>(backend: &B) -> Self {}
    pub fn load<B: Backend>(&self, backend: &B) -> anyhow::Result<()> {
        let current_song = &self.tracks[self.current_index];
        backend.load(&current_song.uri)?;
        Ok(())
    }

    pub fn play_next<B: Backend>(&mut self, backend: &B) -> anyhow::Result<()> {
        if self.current_index + 1 < self.tracks.len() {
            self.current_index += 1;
            self.load(backend)?;
        }
        Ok(())
    }

    pub fn play_previous<B: Backend>(&mut self, backend: &B) -> anyhow::Result<()> {
        if self.current_index > 0 {
            self.current_index -= 1;
            self.load(backend)?;
        }
        Ok(())
    }
}
