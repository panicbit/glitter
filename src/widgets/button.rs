use rustbox::{RustBox, Color, RB_NORMAL, RB_REVERSE};
use unicode_width::UnicodeWidthStr;

use ::traits::Drawable;

pub struct Button {
	pub text: String,
	pub clicked: bool,
	pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

impl Button {
	pub fn new(text: &str) -> Button {
		Button {
			text: text.to_string(),
			clicked: false,
			horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
		}
	}

	pub fn toggle(&mut self) {
		self.clicked = !self.clicked;
	}
}

impl Drawable for Button {
	fn draw_at(&self, rb: &RustBox, x: usize, y: usize, width: usize, height: usize) {
		let width = UnicodeWidthStr::width_cjk(&*self.text);

		let clicked = match self.clicked {
			true => RB_REVERSE,
			false => RB_NORMAL
		};

		let print = |x, y, ch| rb.print_char(x, y, clicked, Color::Default, Color::Default, ch);

        for x in x..(x+width+1) {
            print(x, y, self.horizontal);
            print(x, y+height, self.horizontal);
        }

        for y in y..(y+height) {
            print(x, y, self.vertical);
            print(x+width+1, y, self.vertical);
        }

        for (index, value) in self.text.chars().enumerate() {
        	print(x, y, self.horizontal);
        	print(x+index+1, y+1, value);
        }

        print(x, y, self.top_left);
        print(x+width+1, y, self.top_right);
        print(x, y+height, self.bottom_left);
        print(x+width+1, y+height, self.bottom_right);
	}
}