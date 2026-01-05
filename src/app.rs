use crossbeam_channel::unbounded;
use std::path::PathBuf;
use std::{thread, time::Duration};

use crate::audio::engine::AudioEngine;
use crate::controller::player::{AudioCommand, AudioEvent, Controller, PlayerState};
use crate::ui::wiremann::Wiremann;
use gpui::*;
use gpui_component::{button::*, *};

pub fn run() {
    let (audio_tx, audio_rx) = unbounded::<AudioCommand>();
    let (events_tx, event_rx) = unbounded::<AudioEvent>();

    thread::spawn(move || {
        AudioEngine::run(audio_rx, events_tx);
    });

    let controller = Controller::new(audio_tx, event_rx, PlayerState::default());

    let app = Application::new();

    app.run(move |cx| {
        gpui_component::init(cx);

        cx.spawn(async move |cx| {
            cx.open_window(WindowOptions::default(), |window, cx| {
                let view = cx.new(|_| Wiremann);
                cx.new(|cx| Root::new(view, window, cx))
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });

    loop {
        std::thread::park();
    }
}

fn sleep(secs: u64) {
    std::thread::sleep(Duration::from_secs(secs));
}
