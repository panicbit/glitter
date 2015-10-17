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
    // Init rustbox
    let rb = RustBox::init(Default::default()).unwrap();
    rb.set_input_mode(InputMode::EscMouse);

    // The application model
    struct Model {
        progress: i64,
        is_progressing: bool
    }

    let mut model = Model {
        progress: 0,
        is_progressing: true
    };


    // Main frame

    let mut frame = Frame::new(|_, _| {});

    // ### Main layout ###

    let mut layout = VerticalLayout::new(|_, _| {});
    layout.set_spacing(1);

    // ### Spinner ###

    let mut spinner = Spinner::new(|this: &mut Spinner<_>, model: &Model| {
        this.rainbow(true);
    });

    let spinner_label = Label::new(|this: &mut Label<_>, model: &Model| {
        this.set_text("Spinning...");
    });

    let mut spinner_layout = HorizontalLayout::new(|_, _| {});
    spinner_layout.add(spinner);
    spinner_layout.add(spinner_label);

    layout.add(spinner_layout);

    // ### Progress bar ###

    let mut progress_layout = HorizontalLayout::new(|_, _| {});

    let mut progress_bar = Progress::new(|this: &mut Progress<_>, model: &Model| {
        this.set_min(0);
        this.set_max(100);
        this.set_value(model.progress);
    });

    let mut progress_percent = Label::new(|this: &mut Label<_>, model: &Model| {
        this.set_text(format!("{:>3}%", model.progress));
    });

    progress_layout.add(progress_percent);
    progress_layout.add(progress_bar);

    layout.add(progress_layout);

    // ### Checkbox ###

    let checkbox = Checkbox::new(|this: &mut Checkbox<_>, model: &Model| {
        this.set_checked(model.is_progressing);
    });

    let checkbox_label = Label::new(|this: &mut Label<_>, model: &Model| {
        this.set_text("Paused")
    });

    let mut checkbox_layout = HorizontalLayout::new(|_, _| {});
    checkbox_layout.add(checkbox);
    checkbox_layout.add(checkbox_label);

    layout.add(checkbox_layout);

    // ### Buttons ###

    let mut button1 = Button::new(|this: &mut Button<_>, model: &Model| {
        this.set_text("I'm a button ガ")
    });
    let mut button2 = Button::new(|this: &mut Button<_>, model: &Model| {
        this.set_text("I'm another button")
    });

    let mut button_layout = HorizontalLayout::new(|_, _| {});
    button_layout.add(button1);
    button_layout.add(button2);

    layout.add(button_layout);

    frame.add(layout);

    loop {
        rb.clear();

        frame.update(&model);
        frame.draw_at(&rb, &model, 1, 1, 45, 20);

        rb.present();

        if model.is_progressing {
            model.progress += 1;

            if model.progress > 100 {
                model.progress = 0;
            }
        }

        //match rb.poll_event(false) {
        match rb.peek_event(Duration::milliseconds(100), false) {
            Ok(Event::KeyEvent(Some(Key::Esc))) => break,
            Ok(Event::KeyEvent(Some(Key::Char(' ')))) => model.is_progressing = !model.is_progressing,
            Ok(ref event) => {
                frame.handle_event(&model, event);
            }
            Err(e) => panic!("{}", e),
        }
    }

}
