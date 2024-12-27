use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::meta::MetadataOptions;
use symphonia::core::codecs::{Decoder, DecoderOptions};
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::probe::Hint;
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::thread;

fn main() {
    let host = cpal::default_host();
    let device = host.default_output_device().expect("No output device available");
    let config = device.default_output_config().unwrap();
    let sample_rate = config.sample_rate().0 as u32;

    let volume = Arc::new(Mutex::new(1.0));

    let file = File::open("path/to/your/song.mp3").expect("Failed to open file");
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    let probed = symphonia::default::get_probe().format(
        &Hint::new(),
        mss,
        &FormatOptions::default(),
        &MetadataOptions::default(),
    ).expect("Failed to probe format");
    let mut format = probed.format;

    let track = format.tracks().iter().next().expect("No tracks found");
    let codec_params = &track.codec_params;
    let mut decoder = symphonia::default::get_codecs().make(codec_params, &DecoderOptions::default()).expect("Failed to create decoder");

    // Create audio stream
    let stream = device.build_output_stream(
        &config.into(),
        {
            let volume = volume.clone();
            move |output: &mut [f32], _| {
                for frame in output.chunks_mut(2) {
                    if let Some(audio_buffer) = fetch_next_audio_frame(&mut format, &mut decoder) {
                        let volume = *volume.lock().unwrap();
                        for (i, sample) in frame.iter_mut().enumerate() {
                            if i < audio_buffer.chan_count() as usize {
                                *sample = audio_buffer.samples()[i] as f32 * volume;
                            }
                        }
                    }
                }
            }
        },
        |err| eprintln!("Error occurred: {}", err),
    ).unwrap();

    stream.play().unwrap();

    // Dynamic volume control in a separate thread
    thread::spawn({
        let volume = volume.clone();
        move || loop {
            let new_volume = 0.5; // Adjust dynamically as needed
            *volume.lock().unwrap() = new_volume;
            thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // Keep the main thread alive while the audio plays
    std::thread::park();
}

fn fetch_next_audio_frame(
    format: &mut Box<dyn FormatReader>,
    decoder: &mut Box<dyn Decoder>,
) -> Option<AudioBufferRef> {
    while let Ok(packet) = format.next_packet() {
        if let Ok(decoded) = decoder.decode(&packet) {
            if let Some(AudioBufferRef::F32(buffer)) = decoded {
                return Some(buffer);
            }
        }
    }
    None
}
