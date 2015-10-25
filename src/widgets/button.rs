use std::rc::Rc;
use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    RB_REVERSE,
    Event,
    Mouse
};

use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    //ActionSender
};
use ::widgets::{Base, Label};

pub type Action = ();

pub struct Button<M> {
    base: Rc<Base<Button<M>, M, Action>>,
    pub label: Label<()>,
    pub x: i32,
    pub y: i32,
    pub clicked: bool,
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char,
}

impl <M> Button<M> {
    pub fn new(model: M) -> Button<M> {
        Button {
            base: Base::new(model),
            label: Label::new(()),
            x: 0,
            y: 0,
            clicked: false,
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘',
        }
    }

    pub fn set_update_handler<F: Fn(&mut Button<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        self.label.set_text(text);
    }

    pub fn toggle(&mut self) {
        self.clicked = !self.clicked;
    }

    pub fn set_action_handler<H: Fn(&mut M, Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }

    pub fn do_action(&mut self, _model: &mut M, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for Button<M> {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, width: usize, height: usize) {
        if width == 0 || height == 0 { return }
        let height = height - 1; // Because drawing at `height + width` is off by one

        let width = self.label.width();
        let clicked = match self.clicked {
            true => RB_REVERSE,
            false => RB_NORMAL
        };

        let print = |x, y, ch| rb.print_char(x, y, clicked, Color::Default, Color::Default, ch);

        for x in x..(x + width + 1) {
            print(x, y, self.horizontal);
            print(x, y + height, self.horizontal);
        }

        for y in y..(y + height) {
            print(x, y, self.vertical);
            print(x + width + 1, y, self.vertical);
        }

        self.label.draw_at(rb, x + 1, y + 1, width, height);

        print(x, y, self.top_left);
        print(x + width + 1, y, self.top_right);
        print(x, y + height, self.bottom_left);
        print(x + width + 1, y + height, self.bottom_right);
    }

    fn width(&self) -> usize {
        let label_width = self.label.width();
        let border_width = 2;

        label_width + border_width
    }

    fn height(&self) -> usize {
        let label_height = self.label.height();
        let border_height = 2;

        label_height + border_height
    }
}

impl <M> EventReceiver for Button<M> {
    fn handle_event(&mut self, event: &Event) -> bool {
        match *event {
            Event::MouseEvent(Mouse::Left, x, y) => {
                let width = self.width() as i32;
                let height = self.height() as i32;
                if x >= self.x && y >= self.y
                    && x < self.x + width
                    && y < self.y + height
                {
                    self.toggle();
                    true
                } else {
                    true
                } //Should add height
            }
            Event::MouseEvent(Mouse::Release, _x, _y) => {
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

impl <M> Widget for Button<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
    }
}
