use anyhow::Error;
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

    Arc::new(Mutex::new(
        gstreamer::ElementFactory::make("playbin")
            .name("playbin")
            .property("uri", uri)
            .build()
            .expect("Could not initialize playbin."),
    ))
}

fn app() -> Element {
    let playbin = setup_gstreamer();
    let mut volume = use_signal(|| 100.0);

    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",

                Button {
                    onclick: {
                        let playbin = Arc::clone(&playbin);
                        move |_| {
                            playbin
                                .lock()
                                .expect("Couldn't lock playbin.")
                                .set_state(gstreamer::State::Playing)
                                .expect("Couldn't set playbin state to playing.");
                        }
                    },
                    label { "Play" }
                }

                Button {
                    onclick: {
                        let playbin = Arc::clone(&playbin);
                        move |_| {
                            playbin
                                .lock()
                                .expect("Couldn't lock playbin.")
                                .set_state(gstreamer::State::Paused)
                                .expect("Couldn't set playbin state to paused.");
                        }
                    },
                    label { "Pause" }
                }
                
                Slider {
                    value: *volume.read(),
                    onmoved: move |v| {
                        volume.set(v);
                        let playbin = Arc::clone(&playbin);
                        playbin
                            .lock()
                            .expect("Couldn't lock playbin.")
                            .set_property("volume", v/100.0);
                    }
                }
            }
        }
    )
}
