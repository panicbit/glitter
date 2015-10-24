extern crate glitter;
mod common;
use common::*;
use glitter::widgets::{Label,Frame};
use glitter::traits::*;

#[test]
#[ignore]
/// # Expected
/// The text "Success!" is displayed inside a frame
fn tui_label_cutoff() {
    simple_test(|rb| {
        let mut label = Label::new(());
        label.set_text("Success!Failure!");
        let mut frame = Frame::new(());
        frame.add(label);
        frame.draw_at(rb, 0, 0, 11, 4);
        rb.present();
    })
}
