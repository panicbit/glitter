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

pub struct HorizontalLayout {
    children: Vec<Box<Widget>>,
    spacing: usize
}

impl HorizontalLayout {
    pub fn new() -> HorizontalLayout {
        HorizontalLayout {
            children: Vec::new(),
            spacing: 0
        }
    }

    pub fn add<W: Widget + 'static>(&mut self, widget: W) {
        self.children.push(Box::new(widget))
    }

    pub fn set_spacing(&mut self, spacing: usize) {
        self.spacing = spacing
    }
}

impl Drawable for HorizontalLayout {
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

impl EventReceiver for HorizontalLayout {
    fn handle_event(&mut self, event: &Event) -> bool {
        // TODO: implement cursor
        for child in &mut self.children {
            child.handle_event(event);
        }

        true
    }
}
