use super::{now_playing::*, titlebar::Titlebar};
use crate::layout::Layout;
use backend::{Backend, playback::Playlist};
use components::{button::Button, slider::Slider, theme::Theme};
use gpui::*;
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Reyvr {
    pub backend: Arc<dyn Backend>,
    pub playlist: Arc<Mutex<Playlist>>,
    pub volume: f64,
    pub vol_slider: View<Slider>,
    pub layout: Layout,
    pub now_playing: NowPlaying,
    pub theme: Theme,
}

impl Render for Reyvr {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let theme = self.theme.clone();
        let now_playing = cx.new_model(|_cx| self.now_playing.clone());
        let titlebar = cx.new_view(|_| Titlebar::new(now_playing.clone(), theme.clone()));

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
                    .bg(theme.background)
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child(Button::new().text("Play").on_click({
                        let app = self.clone();
                        let now_playing = now_playing.clone();
                        let playlist = app.playlist.clone();
                        move |_, cx| {
                            if playlist
                                .lock()
                                .expect("Could not lock playlist")
                                .tracks
                                .len()
                                != 0
                            {
                                if playlist.lock().expect("Could not lock playlist").playing
                                    == false
                                {
                                    if playlist.lock().expect("Could not lock playlist").loaded
                                        == false
                                    {
                                        playlist
                                            .lock()
                                            .expect("Could not lock playlist")
                                            .load(&app.backend.clone())
                                            .expect("Could not load current track.");
                                        app.backend.play().expect("Could not play");
                                    } else {
                                        app.backend.play().expect("Could not play");
                                    }
                                    playlist.lock().expect("Could not lock playlist").playing =
                                        true;
                                }

                                now_playing.update(cx, |np, cx| {
                                    let playlist =
                                        playlist.lock().expect("Could not lock playlist");
                                    let track = playlist.tracks[playlist.current_index].clone();
                                    np.update(
                                        cx,
                                        track.title.into(),
                                        track.album.into(),
                                        track.artists.iter().map(|s| s.clone().into()).collect(),
                                    );
                                    cx.notify();
                                });
                            } else {
                                println!("Playlist is not loaded.");
                            }
                        }
                    }))
                    .child(Button::new().text("Pause").on_click({
                        let app = self.clone();
                        let playlist = app.playlist.clone();

                        move |_, _| {
                            if playlist.lock().expect("Could not lock playlist").playing == true {
                                app.backend.pause().expect("Could not pause playback");
                            }
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
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .h_auto()
                            .w_20()
                            .child(self.vol_slider.clone()),
                    )
                    .child(div().w_4().child(format!("{}", self.volume.clone()))),
            )
    }
}
