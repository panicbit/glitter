extern crate rustbox;
use self::rustbox::{
    RustBox,
    Event
};

pub fn simple_test<F: Fn(&RustBox)>(test: F) {
    let rb = &init_rustbox();
    test(rb);
    wait_for_key(rb);
}

pub fn init_rustbox() -> RustBox {
    RustBox::init(Default::default()).unwrap()
}

pub fn wait_for_key(rb: &RustBox) {
    loop  {
        match rb.poll_event(false) {
            Ok(Event::KeyEvent(_)) => break,
            Err(e) => panic!("{}", e),
            _ => {}
        }
    }
}
