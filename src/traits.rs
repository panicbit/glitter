use rustbox::{
    RustBox,
    Event
};

pub trait Drawable<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x_pos: usize, y_pos: usize, available_width: usize, available_height: usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait EventReceiver<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool;
}

pub trait Widget<M>: Drawable<M> + EventReceiver<M> {
    fn update(&mut self, model: &M);
}

/*
pub trait ActionSender<M> {
    type Action;
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H);
    fn do_action(&mut self, model: &mut M, action: Self::Action);
}
*/
