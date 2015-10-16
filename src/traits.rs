use rustbox::{
    RustBox,
    Event
};

pub trait Drawable {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize);
}

pub trait EventReceiver {
    fn handle_event(&mut self, event: &Event) -> bool;
}
