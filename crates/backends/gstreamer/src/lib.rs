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
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_state(gstreamer::State::Playing)
            .expect("Couldn't set playbin state to playing.");
        Ok(())
    }

    fn pause(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_state(gstreamer::State::Paused)
            .expect("Couldn't set playbin state to paused.");
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_state(gstreamer::State::Null)
            .expect("Couldn't set playbin state to stopped.");
        Ok(())
    }

    fn set_volume(&self, volume: f64) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_property("volume", volume);
        Ok(())
    }

    fn get_volume(&self) -> anyhow::Result<f32> {
        let playbin = Arc::clone(&self.playbin);
        let volume: f32 = playbin
            .lock()
            .expect("Could not lock playbin")
            .property("volume");
        Ok(volume)
    }

    fn get_state(&self) -> anyhow::Result<backend::PlaybackState> {
        let playbin = Arc::clone(&self.playbin);

        match playbin
            .lock()
            .expect("Could not lock playbin")
            .current_state()
        {
            gstreamer::State::Playing => Ok(backend::PlaybackState::Playing),
            gstreamer::State::Paused => Ok(backend::PlaybackState::Paused),
            _ => Ok(backend::PlaybackState::Stopped),
        }
    }
}

impl GstBackend {
    /// Constructor to create a new GstBackend instance.
    pub fn new() -> Result<GstBackend, anyhow::Error> {
        let playbin = gstreamer::ElementFactory::make("playbin")
            .name("playbin")
            .build()
            .map_err(|e| anyhow!("Failed to create playbin: {:?}", e))?;

        Ok(GstBackend {
            playbin: Arc::new(Mutex::new(playbin)),
        })
    }
}
