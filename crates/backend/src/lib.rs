use anyhow::Error;
use std::sync::{Arc, Mutex};

pub trait Backend: Send + Sync {
    /// Initialize the backend.
    fn init() -> anyhow::Result<()>;

    /// Play a track given a URI.
    fn play(&self, uri: &str) -> anyhow::Result<()>;

    /// Pause playback.
    fn pause(&self) -> anyhow::Result<()>;

    /// Stop playback.
    fn stop(&self) -> anyhow::Result<()>;

    /// Get the current playback state.
    fn state(&self) -> anyhow::Result<PlaybackState>;
}

/// Playback state representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}
