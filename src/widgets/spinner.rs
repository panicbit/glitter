use std::rc::Rc;
use std::cell::Cell;
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

pub struct Spinner<M> {
    updater: Rc<Box<Fn(&mut Spinner<M>, &M)>>,
    frame: Cell<u32>,
    rainbow: bool
}

impl <M> Spinner<M> {
    pub fn new<F: Fn(&mut Spinner<M>, &M) + 'static>(updater: F) -> Spinner<M> {
        Spinner {
            updater: Rc::new(Box::new(updater)),
            frame: Cell::new(0),
            rainbow: false,
        }
    }

    fn get_anim_char(&self) -> char {
        match self.frame.get() % 6 {
            0 => '⠋',
            1 => '⠙',
            2 => '⠸',
            3 => '⠴',
            4 => '⠦',
            _ => '⠇'
        }
    }

    fn get_anim_color(&self) -> Color {
        if !self.rainbow {
            return Color::Default
        }

        match self.frame.get() % 7 {
            0 => Color::Default,
            1 => Color::Red,
            2 => Color::Green,
            3 => Color::Yellow,
            4 => Color::Blue,
            5 => Color::Magenta,
            _ => Color::Cyan
        }
    }

    fn increment_frame(&self) {
        // Increment frame number and loop around at u32 border
        self.frame.set((self.frame.get() + 1) % ::std::u32::MAX);
    }

    pub fn rainbow(&mut self, rainbow: bool) {
        self.rainbow = rainbow
    }
}

impl <M> Drawable<M> for Spinner<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x: usize, y: usize, w: usize, h: usize) {
        let ch = self.get_anim_char();
        let color = self.get_anim_color();
        self.increment_frame();
        rb.print_char(x, y, RB_NORMAL, color, Color::Default, ch);
    }

    fn width(&self) -> usize {
        1
    }

    fn height(&self) -> usize {
        1
    }
}

impl <M> EventReceiver<M> for Spinner<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool {
        false
    }
}

impl <M> Widget<M> for Spinner<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model)
    }
}
