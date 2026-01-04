use crate::controller::player::AudioCommand;
use crossbeam_channel::Receiver;
use rodio::{Decoder, OutputStreamBuilder, Sink};
use std::{fs::File, io::BufReader, path::PathBuf};

pub struct AudioEngine {
    sink: Sink,
}

impl AudioEngine {
    pub fn run(rx: Receiver<AudioCommand>) {
        let stream_handle = OutputStreamBuilder::open_default_stream().unwrap();
        let sink = Sink::connect_new(&stream_handle.mixer());

        let mut engine = AudioEngine { sink };

        engine.event_loop(rx);
    }

    fn event_loop(&mut self, rx: Receiver<AudioCommand>) {
        while let Ok(cmd) = rx.recv() {
            match cmd {
                AudioCommand::Load(path) => self.load(PathBuf::from(path)),
                AudioCommand::Play => self.sink.play(),
                AudioCommand::Pause => self.sink.pause(),
                // AudioCommand::Stop => self.stop(),
                // AudioCommand::Seek(pos) => self.seek(pos),
                // AudioCommand::Shutdown => break,
                _ => {}
            }
        }
    }

    fn load(&mut self, path: PathBuf) {
        self.sink.stop();

        let file = File::open(path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        self.sink.append(source);
    }
}
