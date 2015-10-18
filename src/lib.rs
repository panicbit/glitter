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

/// Start main loop
pub fn run<M, W: Widget<M>>(mut model: M, mut widget: W) {
    // Init rustbox
    let rb = RustBox::init(Default::default()).unwrap();
    rb.set_input_mode(InputMode::EscMouse);

    loop {
        rb.clear();

        widget.update(&model);
        widget.draw_at(&rb, &model, 1, 1, 45, 20);

        rb.present();

        //match rb.poll_event(false) {
        match rb.peek_event(Duration::milliseconds(100), false) {
            Ok(Event::KeyEvent(Some(Key::Esc))) => break,
            Ok(ref event) => {
                widget.handle_event(&mut model, event);
            }
            Err(e) => panic!("{}", e),
        }
    }

}
