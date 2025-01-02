use gpui::{Bounds, EntityId, Pixels, Point, Render};

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

#[allow(dead_code)]
impl Slider {
    fn new() -> Self {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 100.0,
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

    fn step(mut self, value: f32) -> Self {
        self.step = value;
        self
    }

    fn default(mut self) -> Self {
        self.value = 0.0;
        self
    }

    fn set(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    fn handle_drag(&mut self, position: Point<Pixels>, cx: &mut gpui::ViewContext<Self>) {
        let bounds = self.bounds;
        let min = self.min;
        let max = self.max;
        let step = self.step;

        let relative = (position.x - bounds.left()) / bounds.size.width;
        let value = ((min + (max - min) * relative) / step).round() * step;
        self.value = value.clamp(self.min, self.max);
    }
}
