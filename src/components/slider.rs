use gpui::{
    Bounds, DragMoveEvent, EntityId, InteractiveElement, IntoElement, MouseButton, MouseDownEvent,
    ParentElement as _, Pixels, Point, Render, StatefulInteractiveElement as _, Styled,
    ViewContext, VisualContext as _, canvas, div, px, relative, rgb,
};

#[derive(Clone, Copy, Render)]
pub struct Thumb(EntityId);

pub struct Slider {
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    bounds: Bounds<Pixels>,
    on_change: Box<dyn Fn(f32) + 'static>,
}

#[allow(dead_code)]
impl Slider {
    pub fn new() -> Self {
        Slider {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 100.0,
            bounds: Bounds::default(),
            on_change: Box::new(|value| println!("Value: {}!", value)),
        }
    }

    pub fn min(mut self, value: f32) -> Self {
        self.min = value;
        self
    }

    pub fn max(mut self, value: f32) -> Self {
        self.max = value;
        self
    }

    pub fn step(mut self, value: f32) -> Self {
        self.step = value;
        self
    }

    pub fn default(mut self) -> Self {
        self.value = 0.0;
        self
    }

    pub fn set(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn relative_value(&self) -> f32 {
        let step = self.step;
        let value = self.value;
        let min = self.min;
        let max = self.max;

        let relative_value = (value - min) / (max - min);
        let relative_step = step / (max - min);

        let relative_value = (relative_value / relative_step).round() * relative_step;
        relative_value.clamp(0.0, 1.0)
    }

    pub fn handle_drag(&mut self, position: Point<Pixels>, _cx: &mut ViewContext<Slider>) {
        let bounds = self.bounds;
        let min = self.min;
        let max = self.max;
        let step = self.step;

        let relative = (position.x - bounds.left()) / bounds.size.width;
        let value =
            (((min + (max - min) * relative) / step).round() * step).clamp(self.min, self.max);
        self.value = value;
        (self.on_change)(self.value);
    }

    pub fn render_thumb(&self, cx: &mut ViewContext<Slider>) -> impl gpui::IntoElement {
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

    pub fn on_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut ViewContext<Slider>) {
        self.handle_drag(event.position, cx);
    }
}

impl Render for Slider {
    fn render(&mut self, cx: &mut ViewContext<Slider>) -> impl IntoElement {
        div()
            .id("slider")
            .on_mouse_down(MouseButton::Left, cx.listener(Slider::on_mouse_down))
            .h_5()
            .child(
                div()
                    .id("slider-bar")
                    .relative()
                    .w_32()
                    .my_1p5()
                    .h_1p5()
                    .bg(rgb(0x313244))
                    .rounded(px(3.))
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .h_full()
                            .w(relative(self.relative_value()))
                            .bg(rgb(0x45475a))
                            .rounded_l(px(3.)),
                    )
                    .child(self.render_thumb(cx))
                    .child({
                        let view = cx.view().clone();
                        canvas(
                            move |bounds, cx| view.update(cx, |r, _| r.bounds = bounds),
                            |_, _, _| {},
                        )
                        .absolute()
                        .size_full()
                    }),
            )
    }
}
