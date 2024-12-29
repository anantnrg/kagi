use gpui::{Div, ElementId, Pixels, SharedString, Styled, WindowContext, div, prelude::*};

pub struct Button {
    base: Div,
    id: ElementId,
    icon: Option<SharedString>,
    label: Option<SharedString>,
    rounded: Pixels,
    size: Pixels,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut WindowContext) + 'static>>,
}

impl Button {
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            base: div(),
            id: id.into(),
            icon: None,
            label: None,
            rounded: 8.0.into(),
            size: 16.0.into(),
            on_click: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<SharedString>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn rounded(mut self, size: impl Into<Pixels>) -> Self {
        self.rounded = size.into();
        self
    }

    pub fn size(mut self, size: impl Into<Pixels>) -> Self {
        self.size = size.into();
        self
    }

    pub fn on_click(mut self, handler: impl Fn(&ClickEvent, &mut WindowContext) + 'static) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    pub fn render(self, cx: &mut WindowContext) -> Div {
        let mut button = self.base;
        button
            .style()
            .padding(self.size)
            .border_radius(self.rounded);

        if let Some(icon) = self.icon {
            button = button.child(div().text(icon).style(|s| s.margin_right(4.0)));
        }
        if let Some(label) = self.label {
            button = button.child(div().text(label));
        }

        if let Some(handler) = self.on_click {
            button = button.on_click(move |evt, ctx| handler(evt, ctx));
        }

        button
    }
}
