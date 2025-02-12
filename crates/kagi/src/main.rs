use anyhow::Error;
use backend::{Backend as _, gstreamer::GstBackend};
use std::sync::Arc;
use ui::run_app;

fn main() -> Result<(), Error> {
    smol::block_on(async {
        GstBackend::init()
            .await
            .expect("Could not initialize GStreamer backend: {e}");
        let backend = GstBackend::new().expect("Could not create GStreamer backend");
        run_app(Arc::new(backend)).expect("Could not run app");
    });
    Ok(())
}
