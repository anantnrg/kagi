use crate::controller::player::{AudioCommand, PlayerState};
use crossbeam_channel::Receiver;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

pub struct AudioEngine {
    sink: Sink,
    stream_handle: OutputStream,
    player_state: PlayerState,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum PlaybackState {
    #[default]
    Stopped,
    Playing,
    Paused,
}

impl AudioEngine {
    pub fn run(rx: Receiver<AudioCommand>) {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream_handle.mixer());

        let mut engine = AudioEngine {
            sink,
            stream_handle,
            player_state: PlayerState::default(),
        };

        engine.event_loop(rx);
    }

    fn event_loop(&mut self, rx: Receiver<AudioCommand>) {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Load(path) => self.load(PathBuf::from(path)),
                AudioCommand::Play => self.play(),
                AudioCommand::Pause => self.pause(),
                AudioCommand::Stop => self.stop(),
                AudioCommand::Volume(vol) => self.set_volume(vol),
                AudioCommand::Seek(pos) => self.seek(pos),
                _ => {}
            }
        }
    }

    fn load(&mut self, path: PathBuf) {
        self.sink.stop();
        self.sink = Sink::connect_new(self.stream_handle.mixer());
        self.player_state.current = Some(path.clone());

        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        self.sink.set_volume(self.player_state.volume);
        self.sink.append(source);

        self.player_state.state = PlaybackState::Paused;
    }

    fn play(&mut self) {
        if self.player_state.state != PlaybackState::Playing {
            self.sink.play();
            self.player_state.state = PlaybackState::Playing;
        }
    }

    fn pause(&mut self) {
        if self.player_state.state == PlaybackState::Playing {
            self.sink.pause();
            self.player_state.state = PlaybackState::Paused;
        }
    }

    fn stop(&mut self) {
        self.sink.stop();
        self.player_state.state = PlaybackState::Stopped;
    }

    fn set_volume(&mut self, volume: f32) {
        self.player_state.volume = volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.player_state.volume);
    }

    fn position(&self) -> Duration {
        self.sink.get_pos()
    }

    fn seek(&mut self, pos: u64) {
        self.sink.try_seek(Duration::from_secs(pos)).unwrap();
    }
}
