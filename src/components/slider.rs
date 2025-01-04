use gpui::{DragMoveEvent, MouseButton, ViewContext, WindowContext, div, prelude::*, px, rgb};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Slider {
    width: f32,
    height: f32,
    thumb_size: f32,
    bg_color: u32,
    thumb_color: u32,
    value: f32,
    on_change: Box<dyn Fn(f64, &mut WindowContext) + 'static>,
}

impl Slider {
    pub fn new() -> Self {
        Self {
            width: 200.0,
            height: 10.0,
            thumb_size: 20.0,
            bg_color: 0x45475a,
            thumb_color: 0xcba6f7,
            value: 0.5,
            on_change: Box::new(|_, _| println!("Value changed!")),
        }
    }

    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    pub fn thumb_size(mut self, size: f32) -> Self {
        self.thumb_size = size;
        self
    }

    pub fn bg(mut self, color: u32) -> Self {
        self.bg_color = color;
        self
    }

    pub fn thumb_color(mut self, color: u32) -> Self {
        self.thumb_color = color;
        self
    }

    pub fn value(mut self, value: f32) -> Self {
        self.value = value.clamp(0.0, 1.0);
        self
    }

    pub fn min(mut self, value: f32) -> Self {
        self.value = self.value.clamp(value, self.value);
        self
    }

    pub fn max(mut self, value: f32) -> Self {
        self.value = self.value.clamp(self.value, value);
        self
    }

    pub fn step(mut self, _step: f32) -> Self {
        // Optional stepping logic
        self
    }

    pub fn on_change<F>(mut self, callback: F) -> Self
    where
        F: Fn(f64, &mut WindowContext) + 'static,
    {
        self.on_change = Box::new(callback);
        self
    }
}

impl Render for Slider {
    fn render(&mut self, cx: &mut ViewContext<Slider>) -> impl IntoElement {
        let width = self.width;
        let height = self.height;
        let thumb_size = self.thumb_size;
        let bg_color = self.bg_color;
        let thumb_color = self.thumb_color;
        let value = Rc::new(RefCell::new(self.value));
        let on_change = &self.on_change;

        let thumb_pos = *value.borrow() * width;

        div()
            .flex()
            .w(px(width))
            .h(px(height))
            .bg(rgb(bg_color))
            .rounded(px(height / 2.0))
            .justify_center()
            .items_center()
            .child({
                let value_clone = Rc::clone(&value);
                div()
                    .absolute()
                    .left(px(thumb_pos - thumb_size / 2.0))
                    .w(px(thumb_size))
                    .h(px(thumb_size))
                    .bg(rgb(thumb_color))
                    .rounded(px(thumb_size / 2.0))
                    .on_mouse_down(MouseButton::Left, move |_, context| {
                        context.stop_propagation();
                    })
                    .on_drag_move(move |e: &DragMoveEvent<f32>, context| {
                        let new_value = (e.event.position.x / width).clamp(px(0.0), px(1.0)).0;
                        if (*value_clone.borrow() - new_value).abs() > f32::EPSILON {
                            *value_clone.borrow_mut() = new_value;
                            on_change(new_value.into(), context);
                        }
                    })
            })
    }
}
