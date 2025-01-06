use anyhow::Error;
use backend::Backend as _;
use gst_backend::GstBackend;
use std::sync::Arc;
use ui::run_app;

fn main() -> Result<(), Error> {
    GstBackend::init().expect("Could not initialize GStreamer backend: {e}");
    let backend = GstBackend::new().expect("Could not create GStreamer backend");
    backend
        .load("file:///D:/repos/reyvr/assets/music.mp3")
        .expect("Could not load URI");
    run_app(Arc::new(backend)).expect("Could not run app");
    Ok(())
}
