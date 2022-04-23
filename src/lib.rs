use rui::*;

// Ultimately we'd like to use swift-bridge, once it's ready.

pub struct AppState {
    cx: Context
}

impl AppState {
    pub fn new() -> Self {
        Self{ cx: Context::new() }
    }
}

#[no_mangle]
pub extern fn new_context() -> *mut AppState {
    Box::into_raw(Box::new(AppState::new()))
}

#[no_mangle]
pub extern fn delete_context(cx: *mut AppState) {
    unsafe { Box::from_raw(cx); }
}
