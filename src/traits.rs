use rustbox::{
    RustBox,
    Event
};

pub trait Drawable {
    fn draw_at(&mut self, rb: &RustBox, x: usize, y: usize, width: usize, height: usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

pub trait EventReceiver {
    fn handle_event(&mut self, event: &Event) -> bool;
}

pub trait Widget: Drawable + EventReceiver {
    fn update(&mut self);
}
