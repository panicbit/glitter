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
    ActionSender
};
use ::widgets::Base;

pub struct HorizontalLayout<M> {
    base: Rc<Base<HorizontalLayout<M>, M>>,
    children: Vec<Box<Widget<M>>>,
    spacing: usize,
}

impl <M> HorizontalLayout<M> {
    pub fn new() -> HorizontalLayout<M> {
        HorizontalLayout {
            base: Base::new(),
            children: Vec::new(),
            spacing: 1,
        }
    }

    pub fn set_update_handler<F: Fn(&mut HorizontalLayout<M>, &M) + 'static>(&mut self, updater: F) {
        self.base.set_update_handler(updater)
    }

    pub fn add<W: Widget<M> + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget))
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing
    }
}

impl <M> Drawable<M> for HorizontalLayout<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x_pos: usize, y_pos: usize, width: usize, height: usize) {
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
            child.draw_at(rb, model, x_pos + x_offset, y_pos, slot_width, height);
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

impl <M> EventReceiver<M> for HorizontalLayout<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(model, event);
        }

        true
    }
}

impl <M> Widget<M> for HorizontalLayout<M> {
    fn update(&mut self, model: &M) {
        self.base.clone().update(self, model);
        for child in &mut self.children {
            child.update(model)
        }

    }
}

impl <M> ActionSender<M> for HorizontalLayout<M> {
    type Action = ();
    fn set_action_handler<H: Fn(&mut M, Self::Action) + 'static>(&mut self, handler: H) {
        self.base.set_action_handler(handler)
    }
    fn do_action(&mut self, model: &mut M, action: Self::Action) {
        self.base.do_action(model, action)
    }
}
