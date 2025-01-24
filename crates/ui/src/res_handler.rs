use backend::player::Response;
use gpui::*;

#[derive(Clone, Copy)]
pub struct ResHandler {}

impl ResHandler {
    pub fn update(&mut self, cx: &mut ModelContext<Self>, res: Response) {
        cx.emit(res);
        cx.notify();
    }
}

impl EventEmitter<Response> for ResHandler {}
