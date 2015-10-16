use rustbox::{
    RustBox, 
    Color, 
    RB_NORMAL, 
    RB_REVERSE,
    Event,
    Key,
    Mouse
};
use unicode_width::UnicodeWidthStr;

use ::traits::{Drawable, EventReceiver};
use ::widgets::Label;

pub struct Button {
    pub label: Label,
    pub x: i32,
    pub y: i32,
    pub clicked: bool,
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
    width: usize,
}

impl Button {
    pub fn new<S: Into<String>>(text: S, x: i32, y: i32) -> Button {
        let label = Label::new(text.into());
        let width = UnicodeWidthStr::width(label.text());
        Button {
            label: label,
            x: x,
            y: y,
            clicked: false,
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
            width: width,
        }
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.label.set_text(text);
        self.width = UnicodeWidthStr::width(self.label.text());
    }

    pub fn toggle(&mut self) {
        self.clicked = !self.clicked;
    }
}

impl Drawable for Button {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, width: usize, height: usize) {
        let width = self.width;
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

        self.label.draw_at(rb, x + 1, y + 1, width, height);

        print(x, y, self.top_left);
        print(x+width+1, y, self.top_right);
        print(x, y+height, self.bottom_left);
        print(x+width+1, y+height, self.bottom_right);
    }
}

impl EventReceiver for Button {
    fn handle_event(&mut self, event: &Event) -> bool {
        match *event {
            Event::MouseEvent(Mouse::Left, x, y) => {
                let width = self.width as i32;
                if x >= self.x && y >= self.y 
                    && x <= self.x+1 + width 
                    && y < self.y + 3i32 
                {
                    self.toggle();
                    true
                } else {
                    true
                } //Should add height
            }
            Event::MouseEvent(Mouse::Release, x, y) => {
                if self.clicked {
                    self.toggle();
                }
                true
            }
            _ => {
                false
            }
        }
    }
}