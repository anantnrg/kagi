use crossbeam_channel::unbounded;
use std::path::PathBuf;
use std::{thread, time::Duration};

use crate::audio::engine::AudioEngine;
use crate::controller::player::AudioCommand;

pub fn run() {
    let (tx, rx) = unbounded::<AudioCommand>();

    thread::spawn(move || {
        AudioEngine::run(rx);
    });

    tx.send(AudioCommand::Load(String::from(
        r"E:\music\violence ft. doomguy\463 - NF - PAID MY DUES.mp3",
    )))
    .unwrap();

    tx.send(AudioCommand::Play).unwrap();
    sleep(3);

    tx.send(AudioCommand::Volume(0.2)).unwrap();

    tx.send(AudioCommand::Pause).unwrap();
    sleep(2);

    tx.send(AudioCommand::Play).unwrap();
    sleep(6);

    tx.send(AudioCommand::Volume(1.0)).unwrap();
    sleep(5);

    tx.send(AudioCommand::Seek(40)).unwrap();
    sleep(5);

    tx.send(AudioCommand::Stop).unwrap();
    sleep(2);

    loop {
        std::thread::park();
    }
}

fn sleep(secs: u64) {
    std::thread::sleep(Duration::from_secs(secs));
}
