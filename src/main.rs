use anyhow::Error;
use glib::MainLoop;
use gstreamer::prelude::*;

fn main() -> Result<(), Error> {
    gstreamer::init()?;

    let uri = "file:///assets/music.mp3";

    let playbin = gstreamer::ElementFactory::make("playbin")
        .name("playbin")
        .property("uri", uri)
        .build()?;

    playbin.set_state(gstreamer::State::Playing)?;

    let main_loop = MainLoop::new(None, false);
    main_loop.run();

    playbin.set_state(gstreamer::State::Null)?;
    Ok(())
}
