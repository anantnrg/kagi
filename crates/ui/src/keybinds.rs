use crate::now_playing::PlayerContext;
use backend::player::Controller;
use gpui::*;
use gstreamer::State;

actions!(kagi, [ChangeState, Prev, Next]);

pub fn register(cx: &mut App) {
    cx.on_action(change_state);
    cx.on_action(next);
    cx.on_action(previous);
    cx.bind_keys([KeyBinding::new("space", ChangeState, None)]);
    cx.bind_keys([KeyBinding::new("ctrl-left", Prev, None)]);
    cx.bind_keys([KeyBinding::new("ctrl-right", Next, None)]);
}

pub fn change_state(_: &ChangeState, cx: &mut App) {
    let state = cx.global::<PlayerContext>().state.read(cx).state;
    let controller = cx.global::<Controller>();
    match state {
        State::Null | State::Paused => controller.play(),
        State::Playing => controller.pause(),
        _ => {}
    }
}

fn next(_: &Next, cx: &mut App) {
    cx.global::<Controller>().next();
}

fn previous(_: &Prev, cx: &mut App) {
    cx.global::<Controller>().prev();
}
