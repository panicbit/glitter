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
    Widget
};

pub struct Checkbox<M> {
    updater: Rc<Box<Fn(&mut Checkbox<M>, &M)>>,
    checked: bool
}

impl <M> Checkbox<M> {
    pub fn new<F: Fn(&mut Checkbox<M>, &M) + 'static>(updater: F) -> Checkbox<M> {
        Checkbox {
            updater: Rc::new(Box::new(updater)),
            checked: false
        }
    }

    pub fn set_checked(&mut self, state: bool) {
        self.checked = state;
    }

    pub fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}

impl <M> Drawable<M> for Checkbox<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x_pos: usize, y_pos: usize, width: usize, height: usize) {
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
            self.toggle();
            true
        } else {
            false
        }
    }
}

impl <M> Widget<M> for Checkbox<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model)
    }
}
