use std::rc::Rc;
use std::cmp::min;
use rustbox::{
    RustBox,
    Event,
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget,
    //ActionSender
};
use ::widgets::Base;

pub type Action = ();

pub struct VerticalLayout<M> {
    base: Rc<Base<VerticalLayout<M>, M, Action>>,
    children: Vec<Box<Widget>>,
    spacing: usize,
}

impl <M> VerticalLayout<M> {
    pub fn new(model: M) -> VerticalLayout<M> {
        VerticalLayout {
            base: Base::new(model),
            children: Vec::new(),
            spacing: 0,
        }
    }

    pub fn set_update_handler<F: Fn(&mut VerticalLayout<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget))
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing
    }

    pub fn set_action_handler<H: Fn(&mut M, Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }

    pub fn do_action(&mut self, model: &mut M, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for VerticalLayout<M> {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize) {
        let mut y_offset = 0;
        for child in self.children.iter() {
            let remaining_height = if y_offset < height {
                height - y_offset
            } else {
                // no remaining vertical space left,
                // so no need to draw remaining children
                break
            };
            let slot_height = min(remaining_height, child.height());
            child.draw_at(rb, x_pos, y_pos + y_offset, width, slot_height);
            y_offset += child.height() + self.spacing;
        }
    }

    fn width(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.width())
            .max()
            .unwrap_or(0)
    }

    fn height(&self) -> usize {
        let children: usize = self.children.iter().map(|child| child.height()).sum();
        let spacing = self.spacing * self.children.len();
        children + spacing
    }
}

impl <M> EventReceiver for VerticalLayout<M> {
    fn handle_event(&mut self, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(event);
        }

        true
    }
}

impl <M> Widget for VerticalLayout<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
        for child in &mut self.children {
            child.update()
        }
    }
}
