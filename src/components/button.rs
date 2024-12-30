use gpui::{
    Bounds, MouseButton, MouseDownEvent, SharedString, ViewContext, WindowContext, div, prelude::*,
    px, rgb, size,
};

use crate::Reyvr;

#[derive(IntoElement)]
pub struct Button {
    text: SharedString,
    width: f32,
    height: f32,
    bg_color: u32,
    text_color: u32,
    border_color: u32,
    rounded: bool,
    on_click: Option<Box<dyn Fn(MouseDownEvent, &mut ViewContext<Reyvr>)>>,
}

impl Button {
    pub fn new() -> Self {
        Self {
            text: SharedString::from("Button"),
            width: 230.0,
            height: 40.0,
            bg_color: 0xcba6f7,
            text_color: 0x1e1e2d,
            border_color: 0x45475a,
            rounded: true,
            on_click: None,
        }
    }

    fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    fn size(mut self, width: f32, height: f32) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    fn bg(mut self, color: u32) -> Self {
        self.bg_color = color;
        self
    }

    fn text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    fn border_color(mut self, color: u32) -> Self {
        self.border_color = color;
        self
    }

    fn rounded(mut self, rounded: bool) -> Self {
        self.rounded = rounded;
        self
    }

    fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(MouseDownEvent, &mut ViewContext<Reyvr>) + 'static,
    {
        self.on_click = Some(Box::new(callback));
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .w(px(self.width))
            .h(px(self.height))
            .bg(rgb(self.bg_color))
            .text_color(rgb(self.text_color))
            .border_2()
            .border_color(rgb(self.border_color))
            .justify_center()
            .content_center()
            .items_center()
            .child(self.text)
            .into_element()
    }
}
