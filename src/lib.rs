#![feature(iter_arith)]
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

    let progress_ruler = Label::new("123456789A");
    let mut progress_percent = Label::new(" 0%");

    // ### Main layout ###

    let mut layout = VerticalLayout::new();

    // ### Spinner ###

    let mut spinner = Spinner::new();
    let spinner_label = Label::new("Spinning...");

    spinner.rainbow(true);

    let mut spinner_layout = HorizontalLayout::new();
    spinner_layout.add(spinner);
    spinner_layout.add(spinner_label);

    layout.add(spinner_layout);

    // ### Checkbox ###

    let checkbox = Checkbox::new(true);
    let checkbox_label = Label::new("I'm a checkbox!");

    let mut checkbox_layout = HorizontalLayout::new();
    checkbox_layout.add(checkbox);
    checkbox_layout.add(checkbox_label);

    layout.add(checkbox_layout);

    // ### Buttons ###

    let mut button1 = Button::new("I'm a button ガ", 1, 6);
    let mut button2 = Button::new("I'm another button", 1, 6);

    let mut button_layout = HorizontalLayout::new();
    button_layout.add(button1);
    button_layout.add(button2);

    layout.add(button_layout);

    let x = 4;
    let y = 20;
    let mut w = 20;
    let mut h = 30;
    let mut progress = 0;
    let mut pbar = Progress::new(progress, 0, 100);

    loop {
        rb.clear();

        Frame::rect().draw_at(&rb, x, y, w, h);

        progress_percent.draw_at(&rb, 1, 1, 0, 1);
        progress_ruler.draw_at(&rb, 1, 2, 0, 1);

        pbar.set_value(progress);
        pbar.draw_at(&rb, 1, 3, 10, 1);

        layout.draw_at(&rb, 1, 5, 50, 10);

        rb.present();

        progress += 1;

        // if progress % 10 == 0 {
        //     button.toggle();
        //     cbox.toggle();
        // }

        if progress > 100 {
            progress = 0;
        }

        progress_percent.set_text(format!("{:>2}%", progress));

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

                layout.handle_event(event);
            }
            Err(e) => panic!("{}", e),
        }
    }

}
