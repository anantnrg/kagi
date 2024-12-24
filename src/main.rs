use rodio::source::{SineWave, Source};
use rodio::{Decoder, OutputStream, Sink};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let source = rodio::Decoder::new(BufReader::new(std::fs::File::open("assets/music.mp3")?))?;
    sink.append(source);

    sink.sleep_until_end();
    Ok(())
}
