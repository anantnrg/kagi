use anyhow::Error;
use glib::MainLoop;
use gstreamer::{ElementFactory, Pipeline, State, prelude::*};

fn main() -> Result<(), Error> {
    // Initialize gstreamerreamer
    gstreamer::init()?;

    let uri = "file:///assets/music.mp3"; // File located in the root directory of the repo

    // Create PlayBin element
    let playbin = gstreamer::ElementFactory::make("playbin")
        .name("playbin")
        // Set URI to play
        .property("uri", uri)
        .build()?;

    // Set the playbin to PLAYING
    playbin.set_state(gstreamer::State::Playing)?;

    // Set up the main loop
    let main_loop = MainLoop::new(None, false);
    main_loop.run();

    // Cleanup
    playbin.set_state(gstreamer::State::Null)?;

    Ok(())
}
