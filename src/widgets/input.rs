use std::rc::Rc;
use rustbox::{
    RustBox, 
    Color, 
    RB_NORMAL,
    RB_REVERSE, 
    Event,
    Key
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

pub struct Input<M> {
    base: Rc<Base<Input<M>, M, Action>>,
    title: String,
    text: String,
}

impl <M> Input<M> {
    pub fn new(model: M) -> Input<M> {
        Input {
            base: Base::new(model),
            text: String::new(),
            title: String::new(),
        }
    }

    pub fn set_update_handler<F: Fn(&mut Input<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn set_title(&mut self, title: &str) {
        self.title = title.to_string();
    }
    
    pub fn set_action_handler<H: Fn(&mut M, Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }

    pub fn do_action(&mut self, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for Input<M> {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, available_width: usize, available_height: usize) {
        let title_width = UnicodeWidthStr::width(self.title());
        rb.print(x, y, RB_NORMAL, Color::Default, Color::Default, &self.title);
        rb.print(x + title_width, y, RB_REVERSE, Color::Default, Color::Default, &self.text);
    }

    fn width(&self) -> usize {
        UnicodeWidthStr::width(self.text())
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver for Input<M> {
    fn handle_event(&mut self, _event: &Event) -> bool {
        match *_event {
            Event::KeyEvent(Some(Key::Backspace)) => {
                self.text.pop();
                true
            }
            Event::KeyEvent(Some(Key::Char(key))) => {
                self.text = self.text.clone() + &key.to_string();
                true
            }
            _ => {
                false
            }
        }
    }
}

impl <M> Widget for Input<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
    }
}