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

pub struct HorizontalLayout<M> {
    base: Rc<Base<HorizontalLayout<M>, M, Action>>,
    children: Vec<Box<Widget>>,
    spacing: usize,
}

impl <M> HorizontalLayout<M> {
    pub fn new(model: M) -> HorizontalLayout<M> {
        HorizontalLayout {
            base: Base::new(model),
            children: Vec::new(),
            spacing: 1,
        }
    }

    pub fn set_update_handler<F: Fn(&mut HorizontalLayout<M>, &M) + 'static>(&mut self, updater: F) {
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

    pub fn do_action(&mut self, action: Action) {
        self.base.do_action(action)
    }
}

impl <M> Drawable for HorizontalLayout<M> {
    fn draw_at(&self, rb: &RustBox, x_pos: usize, y_pos: usize, width: usize, height: usize) {
        let mut x_offset = 0;
        for child in self.children.iter() {
            let remaining_width = if x_offset < width {
                width - x_offset
            } else {
                // no remaining horizontal space left,
                // so no need to draw remaining children
                break
            };
            let slot_width = min(remaining_width, child.width());
            child.draw_at(rb, x_pos + x_offset, y_pos, slot_width, height);
            x_offset += child.width() + self.spacing;
        }
    }

    fn width(&self) -> usize {
        let children: usize = self.children.iter().map(|child| child.width()).sum();
        let spacing = self.spacing * self.children.len();
        children + spacing
    }

    fn height(&self) -> usize {
        self.children
            .iter()
            .map(|child| child.height())
            .max()
            .unwrap_or(0)
    }
}

impl <M> EventReceiver for HorizontalLayout<M> {
    fn handle_event(&mut self, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(event);
        }

        true
    }
}

impl <M> Widget for HorizontalLayout<M> {
    fn update(&mut self) {
        self.base.clone().update(self);
        for child in &mut self.children {
            child.update()
        }

    }
}
