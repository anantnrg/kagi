use anyhow::Error;
use glib::MainLoop;
use gstreamer::prelude::*;
use freya::prelude::*;

fn main() -> Result<(), Error> {
    gstreamer::init()?;

    let uri = "file:///D:/repos/reyvr/assets/music.mp3";

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

fn app() -> Element {
    rsx!(
        Body {
            rect {
                width: "fill",
                height: "fill",
                main_align: "center",
                cross_align: "center",
                Button {
                    onclick: move |_| println!("Button Clicked!"),
                    label { "Button A" }
                }
                FilledButton {
                    onpress: move |_| println!("Button Pressed!"),
                    label { "Button B" }
                }
                OutlineButton {
                    label { "Button C" }
                }
            }
        }
    )
}