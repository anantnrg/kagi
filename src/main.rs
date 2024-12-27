// use anyhow::Error;
// use freya::prelude::*;
// use gstreamer::prelude::*;
// use std::sync::{Arc, Mutex};

// fn main() -> Result<(), Error> {
//     gstreamer::init()?;

//     launch_with_props(app, "Media Player", (400.0, 350.0));

//     Ok(())
// }

// fn setup_gstreamer() -> Arc<Mutex<gstreamer::Element>> {
//     let uri = "file:///D:/repos/reyvr/assets/music.mp3";

//     Arc::new(Mutex::new(
//         gstreamer::ElementFactory::make("playbin")
//             .name("playbin")
//             .property("uri", uri)
//             .build()
//             .expect("Could not initialize playbin."),
//     ))
// }

// fn app() -> Element {
//     let playbin = setup_gstreamer();
//     let mut volume = use_signal(|| 100.0);

//     rsx!(
//         Body {
//             rect {
//                 width: "fill",
//                 height: "fill",
//                 main_align: "center",
//                 cross_align: "center",

//                 Button {
//                     onclick: {
//                         let playbin = Arc::clone(&playbin);
//                         move |_| {
//                             if let Ok(playbin) = playbin.lock() {
//                                 playbin.set_state(gstreamer::State::Playing)
//                                     .expect("Couldn't set playbin state to playing.");
//                                 playbin.set_property("volume", 25.0 / 100.0);
//                             }
//                         }
//                     },
//                     label { "Play" }
//                 }

//                 Button {
//                     onclick: {
//                         let playbin = Arc::clone(&playbin);
//                         move |_| {
//                             playbin
//                                 .lock()
//                                 .expect("Couldn't lock playbin.")
//                                 .set_state(gstreamer::State::Paused)
//                                 .expect("Couldn't set playbin state to paused.");
//                         }
//                     },
//                     label { "Pause" }
//                 }

//                 Slider {
//                     value: *volume.read(),
//                     onmoved: move |p| {
//                         volume.set(p);
//                         let playbin = Arc::clone(&playbin);
//                         if let Ok(playbin) = playbin.lock() {
//                             playbin.set_property("volume", p / 100.0);
//                             println!("volume set to {}", p);
//                         } else {
//                             eprintln!("Failed to lock playbin for volume update.");
//                         }
//                     }
//                 }
//             }
//         }
//     )
// }

use anyhow::Error;
use awedio::Sound;


fn main() -> Result<(), Error> {
    let (mut manager, backend) = awedio::start()?;
    let (sound, mut controller) = awedio::sounds::open_file("assets/music.mp3")?
        .with_adjustable_volume_of(0.25)
        .pausable()
        .controllable();
    manager.play(Box::new(sound));
    controller.set_volume(0.5);

    Ok(())
}
