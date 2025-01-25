use crate::player::Response;

use super::{Backend, playback::Track};
use anyhow::anyhow;
use async_trait::async_trait;
use gstreamer::{ClockTime, MessageView, SeekFlags, State, prelude::*};
use gstreamer_pbutils as gst_pbutils;
use std::sync::{Arc, Mutex};

#[derive(Debug)]
pub struct GstBackend {
    pub playbin: Arc<Mutex<gstreamer::Element>>,
}

#[async_trait]
impl Backend for GstBackend {
    async fn init() -> anyhow::Result<()> {
        gstreamer::init().map_err(|e| anyhow!("Failed to initialize Gstreamer backend: {e}"))
    }

    async fn load(&self, uri: &str) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .expect("Could not lock playbin")
            .set_property("uri", uri);
        Ok(())
    }

    async fn play(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Playing)
            .map_err(|e| anyhow!("Could not set playbin state to playing: {e}"))?;
        Ok(())
    }

    async fn pause(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Paused)
            .map_err(|e| anyhow!("Could not set playbin state to paused: {e}"))?;
        Ok(())
    }

    async fn stop(&self) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_state(gstreamer::State::Null)
            .map_err(|e| anyhow!("Could not set playbin state to stopped: {e}"))?;
        Ok(())
    }

    async fn set_volume(&self, volume: f64) -> anyhow::Result<()> {
        let playbin = Arc::clone(&self.playbin);
        playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .set_property("volume", volume);
        Ok(())
    }

    async fn get_volume(&self) -> anyhow::Result<f32> {
        let playbin = Arc::clone(&self.playbin);
        let volume: f32 = playbin
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .property("volume");
        Ok(volume)
    }

    async fn get_state(&self) -> anyhow::Result<State> {
        let state = self
            .playbin
            .clone()
            .lock()
            .map_err(|e| anyhow::anyhow!("Could not lock playbin: {e}"))?
            .current_state();

        Ok(state)
    }

    async fn get_meta(&self, uri: &str) -> anyhow::Result<Track> {
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
            duration: info
                .duration()
                .unwrap_or(ClockTime::from_seconds(0))
                .seconds(),
            album_art_uri: None,
        })
    }

    async fn monitor(&self) -> Option<Response> {
        let playbin = self.playbin.lock().expect("Could not lock playbin");
        if let Some(bus) = playbin.bus() {
            while let Some(msg) = bus.pop() {
                return match msg.view() {
                    MessageView::StateChanged(state) => {
                        Some(Response::StateChanged(state.current()))
                    }
                    MessageView::Eos(_) => Some(Response::Eos),
                    MessageView::StreamStart(_) => Some(Response::StreamStart),
                    MessageView::Error(e) => Some(Response::Error(e.to_string())),
                    MessageView::Warning(w) => Some(Response::Warning(w.to_string())),
                    MessageView::Info(i) => Some(Response::Info(i.to_string())),
                    _ => None,
                };
            }
        }
        None
    }

    async fn get_position(&self) -> u64 {
        if let Some(pos) = self
            .playbin
            .lock()
            .expect("Could not lock playbin")
            .query_position::<ClockTime>()
        {
            return pos.seconds();
        }
        0
    }

    async fn seek(&self, time: u64) -> anyhow::Result<()> {
        let playbin = self.playbin.lock().expect("Could not lock playbin");
        playbin
            .seek_simple(
                SeekFlags::FLUSH | SeekFlags::KEY_UNIT,
                ClockTime::from_seconds(time),
            )
            .expect("Could not seek");
        Ok(())
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
