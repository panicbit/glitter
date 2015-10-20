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

pub struct Label<M> {
    base: Rc<Base<Label<M>, M>>,
    text: String,
}

impl <M> Label<M> {
    pub fn new() -> Label<M> {
        Label {
            base: Base::new(),
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
}

impl <M> Drawable<M> for Label<M> {
    fn draw_at(&self, rb: &RustBox, _model: &M, x: usize, y: usize, available_width: usize, available_height: usize) {
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
    fn handle_event(&mut self, _model: &mut M, _event: &Event) -> bool {
        false
    }
}

impl <M> Widget<M> for Label<M> {
    fn update(&mut self, model: &M) {
        self.base.clone().update(self, model);
    }
}
/*
impl <M> ActionSender<M> for Label<M> {
    type Action = ();
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }
    fn do_action(&mut self, model: &mut M, action: Self::Action) {
        self.base.do_action(model, action)
    }
}
*/