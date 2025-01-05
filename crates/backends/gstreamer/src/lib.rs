use anyhow::anyhow;
use backend::Backend;
use gstreamer::prelude::*;
use std::sync::{Arc, Mutex};

pub struct GstBackend {
    pub playbin: Arc<Mutex<gstreamer::Element>>,
}

impl Backend for GstBackend {
    fn init() -> anyhow::Result<()> {
        gstreamer::init().map_err(|e| anyhow!("Failed to initialize Gstreamer backend: {e}"))
    }

    fn load(&self, uri: &str) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_property("uri", uri);
        Ok(())
    }

    fn play(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn pause(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn stop(&self) -> anyhow::Result<()> {
        todo!()
    }

    fn set_volume(&self, volume: f32) -> anyhow::Result<()> {
        todo!()
    }

    fn get_volume(&self) -> anyhow::Result<f32> {
        todo!()
    }

    fn get_state(&self) -> anyhow::Result<backend::PlaybackState> {
        todo!()
    }
}
