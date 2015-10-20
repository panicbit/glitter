use rustbox::{
    RustBox,
    Event
};

pub trait Drawable {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, available_width: usize, available_height: usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait EventReceiver {
    fn handle_event(&mut self, event: &Event) -> bool;
}

pub trait Widget: Drawable + EventReceiver {
    fn update(&mut self);
}

/*
pub trait ActionSender<M> {
    type Action;
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H);
    fn do_action(&mut self, model: &mut M, action: Self::Action);
}
*/
