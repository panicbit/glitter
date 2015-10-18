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

/*s
pub trait ActionSender {
    type Action;
    fn add_action_handler<H: FnMut() + 'static>(&mut self, handler: H);
    fn emit_action(&mut self, action: Self::Action);
}
*/
