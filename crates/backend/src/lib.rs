use std::fmt::Debug;

use async_trait::async_trait;
use playback::Track;
use player::Response;
use ring_channel::RingSender;

pub mod gstreamer;
pub mod playback;
pub mod player;

/// Common backend trait. Can be used to implement multple backends.
#[async_trait]
pub trait Backend: Send + Sync + Debug {
    /// Initialize the backend.
    async fn init() -> anyhow::Result<()>
    where
        Self: Sized;

    /// Load a file from given URI.
    async fn load(&self, uri: &str) -> anyhow::Result<()>;

    /// Play playback.
    async fn play(&self) -> anyhow::Result<()>;

    /// Pause playback.
    async fn pause(&self) -> anyhow::Result<()>;

    /// Stop playback.
    async fn stop(&self) -> anyhow::Result<()>;

    /// Set the playback volume.
    async fn set_volume(&self, volume: f64) -> anyhow::Result<()>;

    /// Get the playback volume.
    async fn get_volume(&self) -> anyhow::Result<f32>;

    /// Get the current playback state.
    async fn get_state(&self) -> anyhow::Result<PlaybackState>;

    /// Get metadata
    async fn get_meta(&self, uri: &str) -> anyhow::Result<Track>;

    /// Monitor
    async fn monitor(&self) -> Option<Response>;
}

/// Playback state representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}
