use gpui::{Bounds, EntityId, Pixels, Render};

#[derive(Clone, Copy, Render)]
pub struct Thumb(EntityId);

enum Event {
    Change(f32),
}

pub struct Slider {
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    bounds: Bounds<Pixels>,
}

impl Slider {
    fn new() -> Self {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            bounds: Bounds::default(),
        }
    }

    fn min(mut self, value: f32) -> Self {
        self.min = value;
        self
    }

    fn max(mut self, value: f32) -> Self {
        self.max = value;
        self
    }
}
