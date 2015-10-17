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

pub struct Progress {
    value: i64,
    min: i64,
    max: i64,
    width: usize
}

impl Progress {
    pub fn new(value: i64, min: i64, max: i64) -> Progress {
        Progress {
            value: value,
            min: min,
            max: max,
            width: 10,
        }
    }

    pub fn set_value(&mut self, value: i64) {
        self.value = value;
    }
}

impl Drawable for Progress {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize) {
        fn get_sym(n: i64) -> char{
            match n {
                0 => ' ',
                1 => '▏',
                2 => '▎',
                3 => '▍',
                4 => '▌',
                5 => '▋',
                6 => '▊',
                7 => '▉',
                _ => '█',
            }
        };

        let n_subchars = 8.0;

        // normalize
        let max: f64 = (self.max as f64) - (self.min as f64);
        let value: f64 = (self.value as f64) - (self.min as f64);
        let ratio: f64 = value / max;

        let width: f64 = self.width as f64;
        let to_fill = ratio * n_subchars * width;

        let full = (to_fill / n_subchars).floor() as usize;
        let remaining = to_fill % n_subchars;
        let subchar = get_sym(remaining.round() as i64);

        for x_delta in 0 .. full {
            rb.print_char(x_pos + x_delta, y_pos, RB_NORMAL, Color::Default, Color::Default, '█');
        }

        let subchar_x_pos = full;
        if subchar_x_pos <= width as usize {
             rb.print_char(x_pos + subchar_x_pos, y_pos, RB_NORMAL, Color::Default, Color::Default, subchar);
        }
    }

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        // TODO: Have struct contain height
        1
    }
}

impl EventReceiver for Progress {
    fn handle_event(&mut self, event: &Event) -> bool {
        false
    }
}

