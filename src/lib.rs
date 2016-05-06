//! # Glitter
//! 
//! Glitter is a library for making your Rust user interfaces sparkle!
//! You can use Glitter to make terminal-based user interfaces.
//! Currently, Glitter is built on top of
//! [Rustbox](https://github.com/gchp/rustbox).
//! 
//! ## Features
//! Glitter uses frames, and vertical and horizontal layouts with a widget system
//! to build more complex layouts. Some of the widgets that Glitter has include:
//! 
//! * Buttons
//! * Checkboxes
//! * Frames (Widget Containers)
//! * Horizontal Layout
//! * Inputs
//! * Labels
//! * Progress Bars
//! * Spinners
//! * Vertical Layout
//!
//! Glitter allows for a custom data model, and allows each widget to mutate
//! the state of the model synchronously. In Glitter a main operation loop 
//! redraws each widget on screen allowing each to update their state and 
//! be redrawn correctly. The main loop also propagates events to the
//! widgets.
//!
//! Widgets use an update handler to update their state, and an action handler
//! to update the state of the model. The function to run is determined via
//! closures.
//!
//! ## Usage
//!
//! Here's a simple example of an input widget using it's action handler to
//! modify the model based on an Action
//!
//! ```
//! use std::sync::{Arc, RwLock};
//! use glitter::widgets::{Action, Frame, Input, VerticalLayout};
//!
//! // The application model
//! struct AppModel {
//!     title: String,
//! }
//!
//! let model = Arc::new(RwLock::new(AppModel {
//!     title: "".to_string(),
//! }));
//!
//! // Initial Root Widget
//! let mut frame = Frame::new(());
//!
//! // Initial Layout Widget
//! let mut layout = VerticalLayout::new(());
//!
//! // Setup the Input Widget
//! let mut input = Input::new(model.clone());
//! input.set_title("Input: ");
//! 
//! // Create an action handler for it
//! input.set_action_handler(|model, action| {
//!     // Because the model is an `Arc` we need to grab the
//!     // handle to it first with the `write()` call to be
//!     // able to mutate it synchronously
//!     let mut model = model.write().unwrap();
//!
//!     // Match against the action from the widget
//!     match action {
//!         Action::Submitted(text) => {
//!             // Mutate the state of the model based on the action
//!             model.title = text;
//!         },
//!         _ => (),
//!     }
//! });
//!
//! // Add the input to the layout.
//! layout.add(input);
//!
//! // Add the layout to the root frame
//! frame.add(layout);
//!
//! // Run it! Passing in the root frame
//! glitter::run(frame);
//! ```

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

/// Start main loop. 
///
/// The main loop takes a single widget, you can think of this
/// as the root widget. Frames, Horizontal Layouts, and Vertical Layouts take
/// other widgets as children. When the main loop draws the root widget it also
/// draws the children of the root widget, and so on down the tree of widgets.
/// The main loop also propagates events from Rustbox down to the widgets.
///
/// # Examples
///
/// ```
/// use glitter::widgets::{Frame, Spinner, VerticalLayout};
///
/// let mut frame = Frame::new(());
///
/// let mut layout = VerticalLayout::new(());
/// layout.set_spacing(1);
///
/// let spinner = Spinner::new(());
///
/// layout.add(spinner);
/// frame.add(layout);
///
/// glitter::run(frame);
/// ```
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
