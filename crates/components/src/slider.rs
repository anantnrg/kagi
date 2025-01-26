use crate::theme::Theme;
use gpui::*;

#[derive(Clone, Render)]
pub struct Thumb(EntityId);

pub enum SliderEvent {
    Change(f32),
}

pub struct Slider {
    min: f32,
    max: f32,
    step: f32,
    value: f32,
    bounds: Bounds<Pixels>,
    theme: Theme,
}

impl EventEmitter<SliderEvent> for Slider {}

impl Slider {
    pub fn new(theme: Theme) -> Self {
        Self {
            min: 0.0,
            max: 100.0,
            step: 1.0,
            value: 0.0,
            bounds: Bounds::default(),
            theme,
        }
    }

    pub fn min(mut self, min: f32) -> Self {
        self.min = min;
        self
    }

    pub fn max(mut self, max: f32) -> Self {
        self.max = max;
        self
    }

    pub fn step(mut self, step: f32) -> Self {
        self.step = step;
        self
    }

    pub fn default(mut self, value: f32) -> Self {
        self.value = value;
        self
    }

    pub fn value(&mut self, value: f32, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
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

    fn on_drag(&mut self, position: Point<Pixels>, cx: &mut Context<Self>) {
        let bounds = self.bounds;
        let min = self.min;
        let max = self.max;
        let step = self.step;

        let value = {
            let relative = (position.x - bounds.left()) / bounds.size.width;
            min + (max - min) * relative
        };

        let value = (value / step).round() * step;

        self.value = value.clamp(self.min, self.max);
        cx.emit(SliderEvent::Change(self.value));
        cx.notify();
    }

    fn on_mouse_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        self.on_drag(event.position, cx);
    }

    fn render_thumb(&self, cx: &mut Context<Self>) -> impl gpui::IntoElement {
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

                        view.on_drag(e.event.position, cx)
                    }
                }),
            )
            .absolute()
            .top(px(-5.))
            .left(relative(self.relative_value()))
            .ml(-px(8.))
            .size_4()
            .rounded_full()
            .border(px(1.5))
            .border_color(self.theme.secondary)
            .bg(self.theme.accent)
    }
}

impl Render for Slider {
    fn render(&mut self, win: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .id("slider")
            .on_mouse_down(MouseButton::Left, cx.listener(Self::on_mouse_down))
            .h_5()
            .w_full()
            .child(
                div()
                    .id("bar")
                    .relative()
                    .w_full()
                    .my_1p5()
                    .h_1p5()
                    .bg(self.theme.secondary)
                    .active(|this| this.bg(self.theme.accent))
                    .rounded(px(3.))
                    .child(
                        div()
                            .absolute()
                            .top_0()
                            .left_0()
                            .h_full()
                            .w(relative(self.relative_value()))
                            .bg(self.theme.accent)
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
