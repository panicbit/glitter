use std::rc::Rc;
use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    Event
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    //ActionSender
};
use ::widgets::Base;

pub struct Frame<M> {
    base: Rc<Base<Frame<M>, M>>,
    child: Option<Box<Widget>>,
    design: BoxDesign,
}

#[derive(Copy,Clone)]
pub struct BoxDesign {
    pub horizontal: char,
    pub vertical: char,
    pub top_left: char,
    pub top_right: char,
    pub bottom_left: char,
    pub bottom_right: char
}

pub static RECTANGLE_DESIGN: BoxDesign = BoxDesign {
    horizontal: '─',
    vertical: '│',
    top_left: '┌',
    top_right: '┐',
    bottom_left: '└',
    bottom_right: '┘',
};

pub static ROUNDED_DESIGN: BoxDesign = BoxDesign {
    horizontal: '─',
    vertical: '│',
    top_left: '╭',
    top_right: '╮',
    bottom_left: '╰',
    bottom_right: '╯',
};

impl <M> Frame<M> {
    pub fn new(model: M) -> Frame<M> {
        Frame {
            base: Base::new(model),
            child: None,
            design: RECTANGLE_DESIGN,
        }
    }

    pub fn set_update_handler<F: Fn(&mut Frame<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }
    
    pub fn set_design<D: Into<BoxDesign>>(&mut self, design: D) {
        self.design = design.into()
    }

    pub fn add<W: Widget + 'static>(&mut self, child: W) {
        self.child = Some(Box::new(child))
    }
}

impl <M> Drawable for Frame<M> {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, w: usize, h: usize) {
        let print = |x, y, ch| rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, ch);
        let shadow = '░';

        for x in x..(x+w) {
            print(x, y, self.design.horizontal);
            print(x, y+h, self.design.horizontal);
            print(x+1, y+h+1, shadow);
        }

        for y in y..(y+h) {
            print(x, y, self.design.vertical);
            print(x+w, y, self.design.vertical);
            print(x+w+1, y+1, shadow);
        }

        print(x, y, self.design.top_left);
        print(x+w, y, self.design.top_right);
        print(x, y+h, self.design.bottom_left);
        print(x+w, y+h, self.design.bottom_right);

        print(x+w+1, y+h+1, shadow);

        // Render child
        if let Some(ref child) = self.child {
            // Don't render child if too frame small
            let border_size = 3;
            if w < border_size || h < border_size { return }

            let w = w - border_size;
            let h = h - border_size;

            child.draw_at(&rb, x + 1, y + 1, w, h);
        }

    }

    fn width(&self) -> usize {
        // TODO: Have struct contain width
        5
    }

    fn height(&self) -> usize {
        // TODO: Have struct contain width
        5
    }
}

impl <M> EventReceiver for Frame<M> {
    fn handle_event(&mut self, event: &Event) -> bool {
        if let Some(ref mut child) = self.child {
            child.handle_event(event)
        } else {
            false
        }
    }
}

impl <M> Widget for Frame<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
        if let Some(ref mut child) = self.child {
            child.update();
        }
    }
}
/*
impl <M> ActionSender<M> for Frame<M> {
    type Action = ();
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }
    fn do_action(&mut self, model: &mut M, action: Self::Action) {
        self.base.do_action(model, action)
    }
}
*/