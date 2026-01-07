use crate::audio::engine::PlaybackState;
use crossbeam_channel::{Receiver, Sender};
use gpui::*;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Controller {
    pub audio_tx: Sender<AudioCommand>,
    pub event_rx: Receiver<AudioEvent>,
    pub state: PlayerState,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PlayerState {
    pub current: Option<PathBuf>,
    pub state: PlaybackState,
    pub position: u64,
    pub volume: f32,
    pub duration: u64,
}

pub enum AudioCommand {
    Load(String),
    Play,
    Pause,
    Volume(f32),
    Seek(u64),
    Stop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AudioEvent {
    StateChanged(PlayerState),
    TrackEnded,
}

impl Controller {
    pub fn new(
        audio_tx: Sender<AudioCommand>,
        event_rx: Receiver<AudioEvent>,
        state: PlayerState,
    ) -> Controller {
        Controller {
            audio_tx,
            event_rx,
            state,
        }
    }

    pub fn play(&self) {
        let _ = self.audio_tx.send(AudioCommand::Play);
    }

    pub fn pause(&self) {
        let _ = self.audio_tx.send(AudioCommand::Pause);
    }

    pub fn load(&self, path: String) {
        let _ = self.audio_tx.send(AudioCommand::Load(path));
    }

    pub fn volume(&self, volume: f32) {
        let _ = self.audio_tx.send(AudioCommand::Volume(volume / 100.0));
    }

    pub fn seek(&self, secs: u64) {
        let _ = self.audio_tx.send(AudioCommand::Seek(secs));
    }
}

impl gpui::Global for Controller {}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            current: None,
            state: PlaybackState::Stopped,
            position: 0,
            volume: 1.0,
            duration: 0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct ResHandler {}

impl ResHandler {
    pub fn handle(&mut self, cx: &mut Context<Self>, event: AudioEvent) {
        cx.emit(event);
        cx.notify();
    }
}

impl EventEmitter<AudioEvent> for ResHandler {}
