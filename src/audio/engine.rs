use crate::controller::player::AudioCommand;
use crossbeam_channel::Receiver;
use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, time::Duration};

pub struct AudioEngine {
    sink: Sink,
    state: PlaybackState,
    volume: f32,
    stream_handle: OutputStream,
    current_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
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
            state: PlaybackState::Stopped,
            volume: 1.0,
            stream_handle,
            current_path: None,
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
        self.current_path = Some(path.clone());

        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        self.sink.set_volume(self.volume);
        self.sink.append(source);

        self.state = PlaybackState::Paused;
    }

    fn play(&mut self) {
        if self.state != PlaybackState::Playing {
            self.sink.play();
            self.state = PlaybackState::Playing;
        }
    }

    fn pause(&mut self) {
        if self.state == PlaybackState::Playing {
            self.sink.pause();
            self.state = PlaybackState::Paused;
        }
    }

    fn stop(&mut self) {
        self.sink.stop();
        self.state = PlaybackState::Stopped;
    }

    fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.volume);
    }

    fn position(&self) -> Duration {
        self.sink.get_pos()
    }

    fn seek(&mut self, pos: u64) {
        self.sink.try_seek(Duration::from_secs(pos)).unwrap();
    }
}
