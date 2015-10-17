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

pub struct HorizontalLayout<M> {
    updater: Rc<Box<Fn(&mut HorizontalLayout<M>, &M)>>,
    children: Vec<Box<Widget<M>>>,
    spacing: usize,
}

impl <M> HorizontalLayout<M> {
    pub fn new<F: Fn(&mut HorizontalLayout<M>, &M) + 'static>(updater: F) -> HorizontalLayout<M> {
        HorizontalLayout {
            updater: Rc::new(Box::new(updater)),
            children: Vec::new(),
            spacing: 1,
        }
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
    fn handle_event(&mut self, model: &M, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(model, event);
        }

        true
    }
}

impl <M> Widget<M> for HorizontalLayout<M> {
    fn update(&mut self, model: &M) {
        self.updater.clone()(self, model);
        for child in &mut self.children {
            child.update(model)
        }
    }
}
