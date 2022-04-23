use rui::*;

// Ultimately we'd like to use swift-bridge, once it's ready.

pub struct AppState {
    cx: Context
}

// We want to use https://docs.rs/wgpu/0.3.0/wgpu/struct.Instance.html#method.create_surface_from_core_animation_layer

impl AppState {
    pub fn new() -> Self {
        Self{ cx: Context::new() }
    }
}

#[no_mangle]
pub extern fn new_context() -> *mut AppState {
    println!("creating rui context");
    Box::into_raw(Box::new(AppState::new()))
}

#[no_mangle]
pub extern fn delete_context(cx: *mut AppState) {
    println!("deleting rui context");
    unsafe { Box::from_raw(cx); }
}
