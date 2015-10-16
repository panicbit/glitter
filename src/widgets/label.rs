use rustbox::{
    RustBox, 
    Color, 
    RB_NORMAL, 
    RB_REVERSE,
    Event,
    Key,
    Mouse
};
use ::traits::{Drawable, EventReceiver};

pub struct Label {
    text: String
}

impl Label {
    pub fn new<S: Into<String>>(text: S) -> Label {
        Label {
            text: text.into()
        }
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl Drawable for Label {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, width: usize, height: usize) {
        rb.print(x, y, RB_NORMAL, Color::Default, Color::Default, &self.text);
    }
}

impl EventReceiver for Label {
    fn handle_event(&mut self, event: &Event) -> bool {
        false
    }
}