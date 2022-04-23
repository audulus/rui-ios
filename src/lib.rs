use rui::*;

// Ultimately we'd like to use swift-bridge, once it's ready.

#[no_mangle]
pub extern fn new_context() -> *mut Context {
    Box::into_raw(Box::new(Context::new()))
}

#[no_mangle]
pub extern fn delete_context(cx: *mut Context) {
    unsafe { Box::from_raw(cx); }
}
