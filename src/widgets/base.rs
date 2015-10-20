use std::rc::Rc;
use std::cell::{RefCell, Ref, RefMut};
//use ::traits::ActionSender;

pub struct Base<W, M> {
    model: RefCell<M>,
    updater: RefCell<Option<Box<Fn(&mut W, &M)>>>,
    //action_handler: RefCell<Option<Box<Fn(&mut M, <W as ActionSender<M>>::Action)>>>
}

impl <W, M> Base<W, M> {
    pub fn new(model: M) -> Rc<Base<W, M>> {
        Rc::new(Base {
            model: RefCell::new(model),
            updater: RefCell::new(None),
            //action_handler: RefCell::new(None)
        })
    }
    pub fn set_update_handler<H: Fn(&mut W, &M) + 'static>(&self, updater: H) {
        *self.updater.borrow_mut() = Some(Box::new(updater))
    }
    pub fn update(&self, widget: &mut W) {
        if let Some(ref updater) = *self.updater.borrow() {
            updater(widget, &mut *self.model.borrow_mut())
        }
    }
    /*
    pub fn set_action_handler<H: Fn(&mut M, <W as ActionSender<M>>::Action) + 'static>(&self, handler: H) {
        *self.action_handler.borrow_mut() = Some(Box::new(handler))
    }
    pub fn do_action(&self, model: &mut M, action: <W as ActionSender<M>>::Action) {
        if let Some(ref handler) = *self.action_handler.borrow() {
            handler(model, action)
        }
    }
    */
    pub fn get_model(&self) -> Ref<M> {
        self.model.borrow()
    }
    pub fn get_mut_model(&mut self) -> RefMut<M> {
        self.model.borrow_mut()
    }
}
