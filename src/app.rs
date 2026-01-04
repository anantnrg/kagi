use crossbeam_channel::unbounded;
use std::path::PathBuf;
use std::thread;

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

    loop {
        std::thread::park();
    }
}
