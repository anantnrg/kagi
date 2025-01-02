use gpui::{
    Bounds, DragMoveEvent, EntityId, InteractiveElement, MouseButton, MouseDownEvent,
    ParentElement as _, Pixels, Point, Render, StatefulInteractiveElement as _, Styled,
    ViewContext, VisualContext as _, div, prelude::FluentBuilder as _, px, relative, rgb,
};

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

    fn relative_value(&self) -> f32 {
        let step = self.step;
        let value = self.value;
        let min = self.min;
        let max = self.max;

        let relative_value = (value - min) / (max - min);
        let relative_step = step / (max - min);

        let relative_value = (relative_value / relative_step).round() * relative_step;
        relative_value.clamp(0.0, 1.0)
    }

    fn handle_drag(&mut self, position: Point<Pixels>, _cx: &mut ViewContext<Slider>) {
        let bounds = self.bounds;
        let min = self.min;
        let max = self.max;
        let step = self.step;

        let relative = (position.x - bounds.left()) / bounds.size.width;
        let value =
            (((min + (max - min) * relative) / step).round() * step).clamp(self.min, self.max);
        self.value = value;
    }

    fn render_thumb(&self, cx: &mut ViewContext<Slider>) -> impl gpui::IntoElement {
        let entity_id = cx.entity_id();

        div()
            .id("thumb")
            .on_drag(Thumb(entity_id), |drag, _, cx| {
                cx.stop_propagation();
                cx.new_view(|_| drag.clone())
            })
            .on_drag_move(
                cx.listener(move |view, e: &DragMoveEvent<Thumb>, cx| match e.drag(cx) {
                    Thumb(id) => {
                        if *id != entity_id {
                            return;
                        }

                        view.handle_drag(e.event.position, cx)
                    }
                }),
            )
            .absolute()
            .top(px(-5.))
            .left(relative(self.relative_value()))
            .ml(-px(8.))
            .size_4()
            .rounded_full()
            .border_1()
            .bg(rgb(0xfff))
    }
}
