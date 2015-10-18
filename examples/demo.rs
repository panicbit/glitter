extern crate glitter;
use std::sync::{Arc,RwLock};
use std::thread;
use glitter::widgets::*;
use glitter::traits::*;

fn main() {
    // The application model
    struct AppModel {
        progress: i64,
        is_progressing: bool
    }
    type Model = Arc<RwLock<AppModel>>;

    let mut model = Arc::new(RwLock::new(AppModel {
        progress: 0,
        is_progressing: true
    }));

    // Spawn background thread
    {
        let model = model.clone();
        thread::spawn(move || {
            loop {
                thread::sleep_ms(100);
                let mut model = model.write().unwrap();
                if model.is_progressing {
                    model.progress += 1;
                    if model.progress > 100 {
                        model.progress = 0;
                    }
                }
            }
        });
    }

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
        let model = model.read().unwrap();
        this.set_min(0);
        this.set_max(100);
        this.set_value(model.progress);
    });

    let mut progress_percent = Label::new(|this: &mut Label<_>, model: &Model| {
        let model = model.read().unwrap();
        this.set_text(format!("{:>3}%", model.progress));
    });

    progress_layout.add(progress_percent);
    progress_layout.add(progress_bar);

    layout.add(progress_layout);

    // ### Checkbox ###

    let mut checkbox = Checkbox::new(|this: &mut Checkbox<_>, model: &Model| {
        let model = model.read().unwrap();
        this.set_checked(!model.is_progressing);
    });

    checkbox.set_action_handler(|model, _| {
        let mut model = model.write().unwrap();
        model.is_progressing = !model.is_progressing
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

    glitter::run(model, frame);
}