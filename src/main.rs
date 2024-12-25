use gstreamer as gst;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize GStreamer
    gst::init()?;

    // Create a new playbin element
    let playbin = gst::ElementFactory::make("playbin", Some("player"))
        .expect("Failed to create playbin element");

    // Set the file URI (use `file://` for local files)
    let filepath = "assets/music.mp3";
    playbin.set_property("uri", format!("file://{}", std::fs::canonicalize(filepath)?.display()));

    // Start playback
    playbin.set_state(gst::State::Playing)?;

    println!("Playing: {}", filepath);

    // Wait until playback is finished
    let bus = playbin.bus().unwrap();
    for msg in bus.iter_timed(gst::ClockTime::NONE) {
        use gst::MessageView;

        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of stream");
                break;
            }
            MessageView::Error(err) => {
                eprintln!("Error: {}", err.error());
                break;
            }
            _ => (),
        }
    }

    // Shut down
    playbin.set_state(gst::State::Null)?;

    Ok(())
}
