use std::rc::Rc;
use std::cmp::min;
use rustbox::{
    RustBox,
    Color,
    RB_NORMAL,
    Event,
    Key
};
use ::traits::{
    Drawable,
    EventReceiver,
    Widget
};

pub struct VerticalLayout<M> {
    updater: Rc<Box<Fn(&mut VerticalLayout<M>, &M)>>,
    children: Vec<Box<Widget<M>>>,
    spacing: usize,
}

impl <M> VerticalLayout<M> {
    pub fn new<F: Fn(&mut VerticalLayout<M>, &M) + 'static>(updater: F) -> VerticalLayout<M> {
        VerticalLayout {
            updater: Rc::new(Box::new(updater)),
            children: Vec::new(),
            spacing: 0,
        }
    }

    pub fn add<W: Widget<M> + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget))
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing
    }
}

impl <M> Drawable<M> for VerticalLayout<M> {
    fn draw_at(&self, rb: &RustBox, model: &M, x_pos: usize, y_pos: usize, width: usize, height: usize) {
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
            child.draw_at(rb, model, x_pos, y_pos + y_offset, width, slot_height);
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

impl <M> EventReceiver<M> for VerticalLayout<M> {
    fn handle_event(&mut self, model: &mut M, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(model, event);
        }

        true
    }
}

impl <M> Widget<M> for VerticalLayout<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model);
        for child in &mut self.children {
            child.update(model)
        }
    }
}
