extern crate rustbox;
extern crate time;
use time::Duration;
use std::cmp::max;
use rustbox::{
    RustBox,
    InputMode,
    RB_NORMAL,
    Color,
    Key,
};

use rustbox::Event::MouseEvent;

pub mod widgets;
use widgets::*;
pub mod traits;
use traits::*;

#[test]
fn demo() {
    run();
}

// demo main-loop-thingy
fn run() {
    let rb = RustBox::init(Default::default()).unwrap();
    rb.set_input_mode(InputMode::EscMouse);

    let mut cbox = Checkbox::new(true);
    let x = 4;
    let y = 10;
    let mut w = 20;
    let mut h = 30;
    let mut progress = 0;
    let mut pbar = Progress::new(progress, 0, 100);
    let mut spinner = Spinner::new();
    spinner.rainbow(true);

    loop {
        rb.clear();
        Frame::rect().draw_at(&rb, x, y, w, h);
        cbox.draw_at(&rb, 5, 5, 0, 0);

        rb.print(1, 1, RB_NORMAL, Color::Default, Color::Default, &format!("{}%", progress));
        rb.print(1, 2, RB_NORMAL, Color::Default, Color::Default, "123456789A");

        pbar.set_value(progress);
        pbar.draw_at(&rb, 1, 3, 10, 1);

        spinner.draw_at(&rb, 0, 0, 1, 1);
        rb.print(2, 0, RB_NORMAL, Color::Default, Color::Default, "Spinning...");

        rb.present();

        progress += 1;
        if progress > 100 {
            progress = 0;
        }

        //match rb.poll_event(false) {
        match rb.peek_event(Duration::milliseconds(100), false) {
            Ok(rustbox::Event::KeyEvent(key)) => {
                match key {
                    Some(Key::Esc) => { break }
                    Some(Key::Char(' ')) => { cbox.toggle() }
                    _ => { }
                }
            }
            Ok(MouseEvent(_, xp, yp)) => {
                w = max(0, xp - (x as i32)) as usize;
                h = max(0, yp - (y as i32)) as usize;
            }
            Err(e) => panic!("{}", e),
            _ => { }
        }
    }

}
