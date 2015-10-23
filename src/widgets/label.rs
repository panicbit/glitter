use std::rc::Rc;
use rustbox::{
    RustBox, 
    Color, 
    RB_NORMAL, 
    Event,
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    //ActionSender
};
use unicode_width::UnicodeWidthStr;
use ::widgets::Base;

pub type Action = ();

pub struct Label<M> {
    base: Rc<Base<Label<M>, M, Action>>,
    text: String,
}

impl <M> Label<M> {
    pub fn new(model: M) -> Label<M> {
        Label {
            base: Base::new(model),
            text: String::new(),
        }
    }

    pub fn set_update_handler<F: Fn(&mut Label<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.text = text.into()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
    
    pub fn set_action_handler<H: Fn(&mut M, Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }

    pub fn do_action(&mut self, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for Label<M> {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, available_width: usize, available_height: usize) {
        rb.print(x, y, RB_NORMAL, Color::Default, Color::Default, &self.text);
    }

    fn width(&self) -> usize {
        self.text().width()
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver for Label<M> {
    fn handle_event(&mut self, _event: &Event) -> bool {
        false
    }
}

impl <M> Widget for Label<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
    }
}
