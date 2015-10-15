use rustbox::{
    RustBox,
    Color,
    RB_NORMAL
};
use ::traits::Drawable;

pub struct Checkbox {
    checked: bool
}

impl Checkbox {
    pub fn new(state: bool) -> Checkbox {
        Checkbox {
            checked: state
        }
    }

    pub fn set_checked(&mut self, state: bool) {
        self.checked = state;
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

impl Drawable for Checkbox {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize) {
        let ch = match self.checked {
            true => '☒',
            false => '☐'
        };
        rb.print_char(x_pos, y_pos, RB_NORMAL, Color::Default, Color::Default, ch);
    }
}