use crate::player_context::PlayerContext;
use backend::player::Controller;
use gpui::*;
use gstreamer::State;

actions!(kagi, [
    ChangeState,
    Prev,
    Next,
    VolUp,
    VolDown,
    SeekForward,
    SeekBackward
]);

pub fn register(cx: &mut App) {
    cx.on_action(change_state);
    cx.on_action(next);
    cx.on_action(previous);
    cx.on_action(vol_up);
    cx.on_action(vol_down);
    cx.on_action(seek_forward);
    cx.on_action(seek_backward);
    cx.bind_keys([
        KeyBinding::new("space", ChangeState, None),
        KeyBinding::new("ctrl-left", Prev, None),
        KeyBinding::new("ctrl-right", Next, None),
        KeyBinding::new("left", SeekBackward, None),
        KeyBinding::new("right", SeekForward, None),
        KeyBinding::new("up", VolUp, None),
        KeyBinding::new("down", VolDown, None),
    ]);
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

fn vol_up(_: &VolUp, cx: &mut App) {
    let state = cx.global_mut::<PlayerContext>().state.clone();

    let curr_vol = cx.global::<PlayerContext>().state.read(cx).volume;
    let new_vol = (curr_vol + 0.05).clamp(0.0, 1.0);

    state.update(cx, |this, cx| {
        this.volume = new_vol.clone();
        this.vol(cx, new_vol);
        cx.notify();
    });
    cx.global::<Controller>().volume(new_vol);
}

fn vol_down(_: &VolDown, cx: &mut App) {
    let state = cx.global_mut::<PlayerContext>().state.clone();

    let curr_vol = cx.global::<PlayerContext>().state.read(cx).volume;
    let new_vol = (curr_vol - 0.05).clamp(0.0, 1.0);

    state.update(cx, |this, cx| {
        this.volume = new_vol.clone();
        this.vol(cx, new_vol);
        cx.notify();
    });
    cx.global::<Controller>().volume(new_vol);
}

fn seek_forward(_: &SeekForward, cx: &mut App) {
    let current_pos = cx.global::<PlayerContext>().state.read(cx).position;
    let total_duration = cx.global::<PlayerContext>().metadata.read(cx).duration;
    cx.global::<Controller>()
        .seek((current_pos + 5).clamp(0, total_duration));
}

fn seek_backward(_: &SeekBackward, cx: &mut App) {
    let current_pos = cx.global::<PlayerContext>().state.read(cx).position;
    cx.global::<Controller>()
        .seek(current_pos.saturating_sub(5));
}
