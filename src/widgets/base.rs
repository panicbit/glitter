use std::rc::Rc;
use std::cell::RefCell;
use ::traits::ActionSender;

pub struct Base<W: ActionSender<M>, M> {
    model: M,
    updater: RefCell<Option<Box<Fn(&mut W, &M)>>>,
    action_handler: RefCell<Option<Box<Fn(&mut M, <W as ActionSender<M>>::Action)>>>
}

impl <W: ActionSender<M>, M> Base<W, M> {
    pub fn new(model: M) -> Rc<Base<W, M>> {
        Rc::new(Base {
            model: model,
            updater: RefCell::new(None),
            action_handler: RefCell::new(None)
        })
    }
    pub fn set_update_handler<H: Fn(&mut W, &M) + 'static>(&self, updater: H) {
        *self.updater.borrow_mut() = Some(Box::new(updater))
    }
    pub fn update(&self, widget: &mut W, model: &M) {
        if let Some(ref updater) = *self.updater.borrow() {
            updater(widget, model)
        }
    }
    pub fn set_action_handler<H: Fn(&mut M, <W as ActionSender<M>>::Action) + 'static>(&self, handler: H) {
        *self.action_handler.borrow_mut() = Some(Box::new(handler))
    }
    pub fn do_action(&self, model: &mut M, action: <W as ActionSender<M>>::Action) {
        if let Some(ref handler) = *self.action_handler.borrow() {
            handler(model, action)
        }
    }
    pub fn get_model(&self) -> &M {
        &self.model
    }
    pub fn get_mut_model(&mut self) -> &mut M {
        &mut self.model
    }
}
