#![feature(iter_arith)]
extern crate rustbox;
extern crate unicode_width;
extern crate unicode_segmentation;
use std::time::Duration;
use rustbox::{
    RustBox,
    InputMode,
    Key,
    Event,
};

pub mod widgets;
pub mod traits;
use traits::*;

/// Start main loop
pub fn run<W: Widget>(mut widget: W) {
    // Init rustbox
    let rb = RustBox::init(Default::default()).unwrap();
    rb.set_input_mode(InputMode::EscMouse);

    loop {
        rb.clear();

        widget.update();
        widget.draw_at(&rb, 1, 1, 45, 20);

        rb.present();

        //match rb.poll_event(false) {
        match rb.peek_event(Duration::from_millis(100), false) {
            Ok(Event::KeyEvent(Key::Esc)) => break,
            Ok(ref event) => {
                widget.handle_event(event);
            }
            Err(e) => panic!("{}", e),
        }
    }

}
