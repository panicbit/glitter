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
    Widget
};

pub struct Frame<M> {
    child: Option<Box<Widget<M>>>,
    design: BoxDesign,
    updater: Rc<Box<Fn(&mut Frame<M>, &M)>>
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
    pub fn new<F: Fn(&mut Frame<M>, &M) + 'static>(updater: F) -> Frame<M> {
        Frame {
            child: None,
            design: RECTANGLE_DESIGN,
            updater: Rc::new(Box::new(updater)),
        }
    }
    
    pub fn set_design<D: Into<BoxDesign>>(&mut self, design: D) {
        self.design = design.into()
    }

    pub fn add<W: Widget<M> + 'static>(&mut self, child: W) {
        self.child = Some(Box::new(child))
    }
}

impl <M> Drawable<M> for Frame<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x: usize, y: usize, w: usize, h: usize) {
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

            child.draw_at(&rb, model, x + 1, y + 1, w, h);
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

impl <M> EventReceiver<M> for Frame<M> {
    fn handle_event(&mut self, model: &M, event: &Event) -> bool {
        if let Some(ref mut child) = self.child {
            child.handle_event(model, event)
        } else {
            false
        }
    }
}

impl <M> Widget<M> for Frame<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model);
        if let Some(ref mut child) = self.child {
            child.update(model);
        }
    }
}
