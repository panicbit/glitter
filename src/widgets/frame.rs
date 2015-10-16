use rustbox::{
    RustBox,
    Color,
    RB_NORMAL
};
use ::traits::Drawable;

pub struct Frame {
    horizontal: char,
    vertical: char,
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char
}

impl Frame {
    pub fn rect() -> Frame {
        Frame {
            horizontal: '─',
            vertical: '│',
            top_left: '┌',
            top_right: '┐',
            bottom_left: '└',
            bottom_right: '┘'
        }
    }
    pub fn rounded() -> Frame {
        Frame {
            horizontal: '─',
            vertical: '│',
            top_left: '╭',
            top_right: '╮',
            bottom_left: '╰',
            bottom_right: '╯'
        }
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
