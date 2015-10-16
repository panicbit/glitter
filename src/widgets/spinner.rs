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
};

pub struct Spinner {
    frame: Cell<u32>,
    rainbow: bool
}

impl Spinner {
    pub fn new() -> Spinner {
        Spinner {
            frame: Cell::new(0),
            rainbow: false
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

impl Drawable for Spinner {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, w: usize, h: usize) {
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

impl EventReceiver for Spinner {
    fn handle_event(&mut self, event: &Event) -> bool {
        false
    }
}
