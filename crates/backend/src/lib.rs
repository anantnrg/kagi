use playback::Track;

pub mod playback;

/// Common backend trait. Can be used to implement multple backends.
pub trait Backend: Send + Sync {
    /// Initialize the backend.
    fn init() -> anyhow::Result<()>
    where
        Self: Sized;

    /// Load a file from given URI.
    fn load(&self, uri: &str) -> anyhow::Result<()>;

    /// Play playback.
    fn play(&self) -> anyhow::Result<()>;

    /// Pause playback.
    fn pause(&self) -> anyhow::Result<()>;

    /// Stop playback.
    fn stop(&self) -> anyhow::Result<()>;

    /// Set the playback volume.
    fn set_volume(&self, volume: f64) -> anyhow::Result<()>;

    /// Get the playback volume.
    fn get_volume(&self) -> anyhow::Result<f32>;

    /// Get the current playback state.
    fn get_state(&self) -> anyhow::Result<PlaybackState>;

    /// Get metadata
    fn get_meta(&self, uri: &str) -> anyhow::Result<Track>;
}

/// Playback state representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}
