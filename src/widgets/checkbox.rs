use std::rc::Rc;
use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    Event,
    Key
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    //ActionSender
};
use ::widgets::Base;

pub enum Action {
    Check,
    Uncheck
}

pub struct Checkbox<M> {
    base: Rc<Base<Checkbox<M>, M, Action>>,
    checked: bool
}

impl <M> Checkbox<M> {
    pub fn new(model: M) -> Checkbox<M> {
        Checkbox {
            base: Base::new(model),
            checked: false
        }
    }

    pub fn set_update_handler<F: Fn(&mut Checkbox<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn set_checked(&mut self, state: bool) {
        self.checked = state;
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }

    pub fn set_action_handler<H: Fn(&mut M, Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }

    pub fn do_action(&mut self, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for Checkbox<M> {
    fn draw_at(&mut self, rb: &RustBox, x: usize, y: usize, _width: usize, _height: usize) {
        let ch = match self.checked {
            true => '☒',
            false => '☐'
        };
        rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, ch);
    }

    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver for Checkbox<M> {
    fn handle_event(&mut self, event: &Event) -> bool {
        if let Event::KeyEvent(Key::Char(' ')) = *event {
            self.do_action(match self.checked {
                true => Action::Check,
                false => Action::Uncheck
            });
            true
        } else {
            false
        }
    }
}

impl <M> Widget for Checkbox<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
    }
}
