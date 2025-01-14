use super::{now_playing::*, titlebar::Titlebar};
use crate::layout::Layout;
use backend::{Backend, playback::Playlist};
use components::button::Button;
use gpui::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Reyvr {
    pub backend: Arc<dyn Backend>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub volume: Arc<Mutex<f64>>,
    pub layout: Layout,
    pub now_playing: NowPlaying,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let volume = Arc::clone(&self.volume);
        let now_playing = cx.new_model(|_cx| self.now_playing.clone());
        let titlebar = cx.new_view(|_| Titlebar::new(now_playing.clone()));

        cx.subscribe(
            &now_playing,
            |this, _, event: &NowPlayingEvent, cx| match event {
                NowPlayingEvent::Update(title, album, artists) => {
                    this.now_playing.title = title.clone();
                    this.now_playing.album = album.clone();
                    this.now_playing.artists = artists.clone();
                    cx.notify();
                }
            },
        )
        .detach();
        div()
            .w_full()
            .h_full()
            .flex_col()
            .child(titlebar.clone())
            .child(
                div()
                    .flex()
                    .gap_8()
                    .bg(rgb(0x1e1e2d))
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child(Button::new().text("Play").on_click({
                        let app = self.clone();
                        let now_playing = now_playing.clone();
                        let playlist = app.playlist.clone();
                        move |_, cx| {
                            playlist
                                .lock()
                                .expect("Could not lock playlist")
                                .load(&app.backend.clone())
                                .expect("Could not load current track.");
                            now_playing.update(cx, |np, cx| {
                                let playlist = playlist.lock().expect("Could not lock playlist");
                                let track = playlist.tracks[playlist.current_index].clone();
                                np.update(
                                    cx,
                                    track.title.into(),
                                    track.album.into(),
                                    track.artists.iter().map(|s| s.clone().into()).collect(),
                                );
                                cx.notify();
                            });
                            app.backend.play().expect("Could not play");
                        }
                    }))
                    .child(Button::new().text("Pause").on_click({
                        let app = self.clone();
                        move |_, _| {
                            app.backend.pause().expect("Could not pause playback");
                        }
                    }))
                    .child(Button::new().text("Load Playlist").on_click({
                        let app = self.clone();
                        let playlist = app.playlist.clone();

                        move |_, _| {
                            let new_playlist = Playlist::from_dir(
                                &app.backend.clone(),
                                PathBuf::from("E:\\music\\PSYCHX - Kordhell, Scarlxrd"),
                            );
                            *playlist.lock().expect("Could not lock playlist") = new_playlist;
                            println!("Playlist loaded");
                        }
                    }))
                    .child(Button::new().text("+").size(40.0, 40.0).on_click({
                        let volume = Arc::clone(&volume);
                        let backend = self.backend.clone();

                        move |_, _| {
                            let mut vol = volume.lock().expect("Could not lock volume");
                            *vol += 0.2;
                            if *vol > 1.0 {
                                *vol = 1.0;
                            }
                            backend.set_volume(*vol).expect("Could not set volume");
                            println!("volume set to: {}", *vol);
                        }
                    }))
                    .child(Button::new().text("-").size(40.0, 40.0).on_click({
                        let backend = self.backend.clone();
                        let volume = Arc::clone(&volume);
                        move |_, _| {
                            let mut vol = volume.lock().expect("Could not lock volume");
                            *vol -= 0.2;
                            if *vol < 0.0 {
                                *vol = 0.0;
                            }
                            backend.set_volume(*vol).expect("Could not set volume");
                            println!("volume set to: {}", *vol);
                        }
                    })),
            )
    }
}
