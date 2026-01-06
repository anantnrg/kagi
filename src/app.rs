use crossbeam_channel::unbounded;
use std::path::PathBuf;
use std::sync::Arc;
use std::{thread, time::Duration};

use crate::audio::engine::AudioEngine;
use crate::controller::player::{AudioCommand, AudioEvent, Controller, PlayerState, ResHandler};
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
                let controller_evt_clone = controller.clone();
                cx.set_global(controller);

                cx.new(|cx| {
                    let res_handler = cx.new(|_| ResHandler {});
                    let arc_res = Arc::new(res_handler.clone());
                    cx.spawn(async move |_, cx| {
                        let res_handler = arc_res.clone();
                        loop {
                            while let Ok(event) = controller_evt_clone.event_rx.try_recv() {
                                res_handler
                                    .update(&mut cx.clone(), |res_handler, cx| {
                                        res_handler.handle(cx, event);
                                    })
                                    .expect("Could not update");
                            }
                            cx.background_executor()
                                .timer(Duration::from_millis(100))
                                .await;
                        }
                    })
                    .detach();

                    cx.subscribe(
                        &res_handler,
                        move |_, _, event: &AudioEvent, cx| match event {
                            AudioEvent::StateChanged(state) => {
                                cx.global_mut::<Controller>().state = state.clone();
                                cx.notify();
                            }
                            _ => (),
                        },
                    )
                    .detach();

                    Root::new(view, window, cx)
                })
            })?;

            Ok::<_, anyhow::Error>(())
        })
        .detach();
    });
}
