use rui::*;

// Ultimately we'd like to use swift-bridge, once it's ready.

use std::alloc::{self, Layout};

#[no_mangle]
pub extern fn new_context() -> *mut Context {
    let ptr = unsafe { alloc::alloc(Layout::new::<Context>()) };
    ptr as *mut Context
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
