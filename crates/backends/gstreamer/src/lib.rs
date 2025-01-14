use anyhow::anyhow;
use backend::{Backend, playback::Track};
use gstreamer::prelude::*;
use gstreamer_pbutils as gst_pbutils;
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
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Playing)
            .map_err(|e| anyhow!("Could not set playbin state to playing: {e}"))?;
        Ok(())
    }

    fn pause(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Paused)
            .map_err(|e| anyhow!("Could not set playbin state to paused: {e}"))?;
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Null)
            .map_err(|e| anyhow!("Could not set playbin state to stopped: {e}"))?;
        Ok(())
    }

    fn set_volume(&self, volume: f64) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_property("volume", volume);
        Ok(())
    }

    fn get_volume(&self) -> anyhow::Result<f32> {
        let playbin = Arc::clone(&self.playbin);
        let volume: f32 = playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .property("volume");
        Ok(volume)
    }

    fn get_state(&self) -> anyhow::Result<backend::PlaybackState> {
        let playbin = Arc::clone(&self.playbin);

        match playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .current_state()
        {
            gstreamer::State::Playing => Ok(backend::PlaybackState::Playing),
            gstreamer::State::Paused => Ok(backend::PlaybackState::Paused),
            _ => Ok(backend::PlaybackState::Stopped),
        }
    }
    fn get_meta(&self, uri: &str) -> anyhow::Result<Track> {
        let discoverer = gst_pbutils::Discoverer::new(gstreamer::ClockTime::from_seconds(10))?;
        let info = discoverer.discover_uri(uri)?;

        let tags = info.tags().unwrap_or_else(|| gstreamer::TagList::new());

        Ok(Track {
            title: tags
                .get::<gstreamer::tags::Title>()
                .and_then(|v| Some(v.get().to_string()))
                .unwrap_or_else(|| uri.to_string()),
            artists: vec![
                tags.get::<gstreamer::tags::Artist>()
                    .and_then(|v| Some(v.get().to_string()))
                    .unwrap_or_else(|| "Unknown Artist".into()),
            ],
            album: tags
                .get::<gstreamer::tags::Album>()
                .and_then(|v| Some(v.get().to_string()))
                .unwrap_or_else(|| "Unknown Album".into()),
            uri: uri.to_string(),
            duration: info.duration().map(|d| d.seconds() as f64),
            album_art_uri: None,
        })
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
