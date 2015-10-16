extern crate rustbox;
extern crate time;
extern crate unicode_width;
use time::Duration;
use std::cmp::max;
use rustbox::{
    RustBox,
    InputMode,
    RB_NORMAL,
    Color,
    Key,
    Event,
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

    let spinner_label = Label::new("Spinning...");
    let progress_ruler = Label::new("123456789A");
    let mut progress_percent = Label::new("0%");

    let mut layout = VerticalLayout::new();

    let mut cbox1 = Checkbox::new(true);
    layout.add(cbox1);
    let mut cbox2 = Checkbox::new(true);
    layout.add(cbox2);

    let mut button = Button::new("I'm a button ガ", 1, 6);
    let x = 4;
    let y = 20;
    let mut w = 20;
    let mut h = 30;
    let mut progress = 0;
    let mut pbar = Progress::new(progress, 0, 100);
    let mut spinner = Spinner::new();
    spinner.rainbow(true);

    loop {
        rb.clear();

        Frame::rect().draw_at(&rb, x, y, w, h);

        spinner_label.draw_at(&rb, 2, 0, 0, 1);
        progress_percent.draw_at(&rb, 1, 1, 0, 1);
        progress_ruler.draw_at(&rb, 1, 2, 0, 1);

        pbar.set_value(progress);
        pbar.draw_at(&rb, 1, 3, 10, 1);

        button.draw_at(&rb, 1, 6, 12, 2);

        spinner.draw_at(&rb, 0, 0, 1, 1);

        layout.draw_at(&rb, 4, 10, 5, 5);

        rb.present();

        progress += 1;

        // if progress % 10 == 0 {
        //     button.toggle();
        //     cbox.toggle();
        // }

        if progress > 100 {
            progress = 0;
        }

        progress_percent.set_text(format!("{}%", progress));

        //match rb.poll_event(false) {
        match rb.peek_event(Duration::milliseconds(100), false) {
            Ok(Event::KeyEvent(Some(Key::Esc))) => break,
            Ok(ref event) => {
                match *event {
                    Event::MouseEvent(_, xp, yp) => {
                        w = max(0, xp - (x as i32)) as usize;
                        h = max(0, yp - (y as i32)) as usize;
                    },
                    _ => (),
                }

                button.handle_event(event);
                layout.handle_event(event);
            }
            Err(e) => panic!("{}", e),
        }
    }

}
