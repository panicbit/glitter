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

pub struct Frame {
    child: Option<Box<Widget>>,
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char
}

impl Frame {
    pub fn rect_with<W: Widget + 'static>(child: W) -> Frame {
        let mut frame = Frame::rect();
        frame.add(child);
        frame
    }
    pub fn rect() -> Frame {
        Frame {
            child: None,
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘'
        }
    }
    pub fn rounded_with<W: Widget + 'static>(child: W) -> Frame {
        let mut frame = Frame::rounded();
        frame.add(child);
        frame
    }
    pub fn rounded() -> Frame {
        Frame {
            child: None,
            horizontal: '─',
            vertical: '│',
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯'
        }
    }
    pub fn add<W: Widget + 'static>(&mut self, child: W) {
        self.child = Some(Box::new(child))
    }
}

impl Drawable for Frame {
    fn draw_at(&self, rb: &RustBox, x: usize, y: usize, w: usize, h: usize) {
        let print = |x, y, ch| rb.print_char(x, y, RB_NORMAL, Color::Default, Color::Default, ch);
        let shadow = '░';

        for x in x..(x+w) {
            print(x, y, self.horizontal);
            print(x, y+h, self.horizontal);
            print(x+1, y+h+1, shadow);
        }

        for y in y..(y+h) {
            print(x, y, self.vertical);
            print(x+w, y, self.vertical);
            print(x+w+1, y+1, shadow);
        }

        print(x, y, self.top_left);
        print(x+w, y, self.top_right);
        print(x, y+h, self.bottom_left);
        print(x+w, y+h, self.bottom_right);

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

impl EventReceiver for Frame {
    fn handle_event(&mut self, event: &Event) -> bool {
        if let Some(ref mut child) = self.child {
            child.handle_event(event)
        } else {
            false
        }
    }
}
