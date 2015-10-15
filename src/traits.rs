use rustbox::RustBox;

pub trait Drawable {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize);
}
