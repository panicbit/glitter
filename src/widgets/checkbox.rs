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

pub struct Checkbox<M> {
    base: Rc<Base<Checkbox<M>, M>>,
    checked: bool
}

impl <M> Checkbox<M> {
    pub fn new() -> Checkbox<M> {
        Checkbox {
            base: Base::new(),
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
}

impl <M> Drawable<M> for Checkbox<M> {
    fn draw_at(&self, rb: &RustBox, _model: &M, x_pos: usize, y_pos: usize, available_width: usize, available_height: usize) {
        let ch = match self.checked {
            true => '☒',
            false => '☐'
        };
        rb.print_char(x_pos, y_pos, RB_NORMAL, Color::Default, Color::Default, ch);
    }

    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver<M> for Checkbox<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool {
        if let Event::KeyEvent(Some(Key::Char(' '))) = *event {
            //self.do_action(model, ());
            true
        } else {
            false
        }
    }
}

impl <M> Widget<M> for Checkbox<M> {
    fn update(&mut self, model: &M) {
        self.base.clone().update(self, model);
    }
}
/*
impl <M> ActionSender<M> for Checkbox<M> {
    type Action = ();
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }
    fn do_action(&mut self, model: &mut M, action: Self::Action) {
        self.base.do_action(model, action)
    }
}
*/