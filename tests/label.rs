extern crate glitter;
mod common;
use common::*;
use glitter::widgets::Label;
use glitter::traits::*;

#[test]
#[ignore]
/// # Expected
/// The text "Success!" is displayed at (1,1)
fn tui_label_cutoff() {
    simple_test(|rb| {
        let mut label = Label::new(());
        label.set_text("Success!Failure!");
        label.draw_at(rb, 1, 1, 8, 1);
        rb.present();
    })
}