use std::rc::Rc;
use rustbox::{
    RustBox, 
    Color, 
    RB_NORMAL, 
    RB_REVERSE,
    Event,
    Key,
    Mouse
};
use ::traits::{Drawable, EventReceiver, Widget};
use unicode_width::UnicodeWidthStr;

pub struct Label<M> {
    updater: Rc<Box<Fn(&mut Label<M>, &M)>>,
    text: String,
}

impl <M> Label<M> {
    pub fn new<F: Fn(&mut Label<M>, &M) + 'static>(updater: F) -> Label<M> {
        Label {
            updater: Rc::new(Box::new(updater)),
            text: String::new(),
        }
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl <M> Drawable<M> for Label<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x: usize, y: usize, width: usize, height: usize) {
        rb.print(x, y, RB_NORMAL, Color::Default, Color::Default, &self.text);
    }

    fn width(&self) -> usize {
        UnicodeWidthStr::width(self.text())
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver<M> for Label<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool {
        false
    }
}

impl <M> Widget<M> for Label<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model)
    }
}
