use gpui::*;
use gpui_component::{button::*, *};

use crate::controller::player::{Controller, PlayerState};

pub struct Wiremann;

impl Render for Wiremann {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .gap_2()
            .size_full()
            .items_center()
            .justify_center()
            .child(
                Button::new("load")
                    .primary()
                    .label("Load")
                    .on_click(|_, _, cx| {
                        cx.global::<Controller>().load(
                            "E:\\music\\violence ft. doomguy\\463 - NF - PAID MY DUES.mp3"
                                .to_string(),
                        )
                    }),
            )
            .child(
                Button::new("play")
                    .primary()
                    .label("Play")
                    .on_click(|_, _, cx| cx.global::<Controller>().play()),
            )
            .child(
                Button::new("pause")
                    .primary()
                    .label("Pause")
                    .on_click(|_, _, cx| cx.global::<Controller>().pause()),
            )
            .child(text::Text::String(SharedString::from(
                cx.global::<Controller>().state.position.to_string(),
            )))
    }
}
