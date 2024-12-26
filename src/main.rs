use anyhow::Error;
use glib::MainLoop;
use gstreamer::prelude::*;
use freya::prelude::*;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), Error> {
    gstreamer::init()?;

    launch_with_props(app, "Button", (400.0, 350.0));

    Ok(())
}

fn setup_gstreamer() -> Arc<Mutex<gstreamer::Element>> {
    let uri = "file:///D:/repos/reyvr/assets/music.mp3";

    Arc::new(Mutex::new(gstreamer::ElementFactory::make("playbin")
        .name("playbin")
        .property("uri", uri)
        .build()
        .expect("Could not initialize playbin.")))
}

fn app() -> Element {
    let playbin = setup_gstreamer();
    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                Button {
                    onclick: move |_| {
                        playbin.lock().expect("Couldn't lock playbin.").set_state(gstreamer::State::Playing).expect("Couldn't set playbin state.");
                    },
                    label { "Button A" }
                }
            }
        }
    )
}