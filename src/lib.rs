use futures::executor::block_on;
use rui::*;

// Ultimately we'd like to use swift-bridge, once it's ready.

pub struct AppState {
    cx: Context,
    setup: Option<Setup>,
}

// We want to use https://docs.rs/wgpu/0.3.0/wgpu/struct.Instance.html#method.create_surface_from_core_animation_layer

impl AppState {
    pub fn new() -> Self {
        Self { cx: Context::new(), setup: None }
    }
}

#[no_mangle]
pub extern "C" fn new_context() -> *mut AppState {
    println!("creating rui context");
    Box::into_raw(Box::new(AppState::new()))
}

#[no_mangle]
pub extern "C" fn delete_context(cx: *mut AppState) {
    println!("deleting rui context");
    unsafe {
        Box::from_raw(cx);
    }
}

struct Setup {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

async fn setup(ca_layer_ptr: *mut core::ffi::c_void) -> Setup {
    let backend = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);

    let instance = wgpu::Instance::new(backend);
    let surface = unsafe { instance.create_surface_from_core_animation_layer(ca_layer_ptr) };
    let adapter =
        wgpu::util::initialize_adapter_from_env_or_default(&instance, backend, Some(&surface))
            .await
            .expect("No suitable GPU adapters found on the system!");

    let adapter_info = adapter.get_info();
    println!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

    let trace_dir = std::env::var("WGPU_TRACE");
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::default(),
                limits: wgpu::Limits::default(),
            },
            trace_dir.ok().as_ref().map(std::path::Path::new),
        )
        .await
        .expect("Unable to find a suitable GPU adapter!");

    Setup {
        surface,
        adapter,
        device,
        queue,
    }
}

#[no_mangle]
pub extern "C" fn setup_surface(cx: *mut AppState, ca_layer_ptr: *mut core::ffi::c_void) {
    let cx = unsafe { cx.as_mut().unwrap() };

    println!("ca_layer_ptr: {:?}", ca_layer_ptr);

    cx.setup = Some(block_on(setup(unsafe { * (ca_layer_ptr as *mut *mut core::ffi::c_void) })));
}
