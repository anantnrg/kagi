use gpui::{MouseButton, MouseDownEvent, SharedString, WindowContext, div, prelude::*, px, rgb};

#[derive(IntoElement)]
pub struct Button {
    text: SharedString,
    w: f32,
    h: f32,
    px: f32,
    bg_color: u32,
    text_color: u32,
    border_color: u32,
    hover_bg_color: u32,
    hover_text_color: u32,
    hover_border_color: u32,
    rounded: f32,
    on_click: Box<dyn Fn(MouseDownEvent, &mut WindowContext) + 'static>,
}

#[allow(dead_code)]
impl Button {
    pub fn new() -> Self {
        Self {
            text: SharedString::from("Button"),
            w: 0.0,
            h: 40.0,
            px: 20.0,
            bg_color: 0x45475a,
            text_color: 0xcdd6f4,
            border_color: 0xcba6f7,
            hover_bg_color: 0xcba6f7,
            hover_text_color: 0x1e1e2d,
            hover_border_color: 0xcba6f7,
            rounded: 8.0,
            on_click: Box::new(|_, _| println!("Clicked!")),
        }
    }

    pub fn text(mut self, text: impl Into<SharedString>) -> Self {
        self.text = text.into();
        self
    }

    pub fn size(mut self, w: f32, h: f32) -> Self {
        self.w = w;
        self.h = h;
        self
    }

    pub fn bg(mut self, color: u32) -> Self {
        self.bg_color = color;
        self
    }

    pub fn text_color(mut self, color: u32) -> Self {
        self.text_color = color;
        self
    }

    pub fn border_color(mut self, color: u32) -> Self {
        self.border_color = color;
        self
    }

    pub fn rounded(mut self, rounded: f32) -> Self {
        self.rounded = rounded;
        self
    }

    pub fn on_click<F>(mut self, callback: F) -> Self
    where
        F: Fn(MouseDownEvent, &mut WindowContext) + 'static,
    {
        self.on_click = Box::new(callback);
        self
    }
}

impl RenderOnce for Button {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        let on_click = self.on_click;
        div()
            .flex()
            .h(px(self.h))
            .when(self.w != 0.0, |this| this.w(px(self.w)))
            .when(self.w == 0.0, |this| this.w_auto().px(px(self.px)))
            .bg(rgb(self.bg_color))
            .text_color(rgb(self.text_color))
            .border_2()
            .rounded(px(self.rounded))
            .border_color(rgb(self.border_color))
            .justify_center()
            .content_center()
            .items_center()
            .child(self.text)
            .hover(move |this| {
                this.bg(rgb(self.hover_bg_color))
                    .text_color(rgb(self.hover_text_color))
                    .border_color(rgb(self.hover_border_color))
            })
            .on_mouse_down(MouseButton::Left, move |event, context| {
                (on_click)(event.clone(), context);
            })
            .into_element()
    }
}
