use core::ffi::c_void;
use futures::executor::block_on;
use rui::*;
use vger::*;

fn my_ui() -> impl View {
    vstack(("Hello", "world"))
}

pub struct AppState {
    cx: Context,
    setup: Option<Setup>,
    config: Option<wgpu::SurfaceConfiguration>,
    vger: Option<Vger>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            cx: Context::new(),
            setup: None,
            config: None,
            vger: None,
        }
    }

    pub fn setup_surface(&mut self, ca_layer_ptr: *mut c_void) {
        println!("ca_layer_ptr: {:?}", ca_layer_ptr);

        let setup = block_on(setup(unsafe { *(ca_layer_ptr as *mut *mut c_void) }));

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: setup.surface.get_preferred_format(&setup.adapter).unwrap(),
            width: 1024,
            height: 768,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        setup.surface.configure(&setup.device, &config);

        self.vger = Some(Vger::new(
            &setup.device,
            wgpu::TextureFormat::Bgra8UnormSrgb,
        ));

        self.setup = Some(setup);
        self.config = Some(config);
    }

    pub fn update(&mut self, width: f32, height: f32) {
        let mut access_nodes = vec![];

        self.cx.update(
            &my_ui(),
            &mut self.vger.as_mut().unwrap(),
            &mut access_nodes,
            [width, height].into(),
        );
    }

    pub fn render(&mut self, width: f32, height: f32, scale: f32) {
        if let Some(setup) = &self.setup {
            self.cx.render(
                &setup.device,
                &setup.surface,
                &self.config.as_ref().unwrap(),
                &setup.queue,
                &my_ui(),
                &mut self.vger.as_mut().unwrap(),
                [width, height].into(),
                scale,
            );
        }
    }

    fn process(&mut self, event: ffi::AppEvent) {
        let position = LocalPoint::from([event.x,event.y]);
        let id = event.id;
        let rui_event = match event.kind {
            ffi::AppEventKind::TouchBegin => Event::TouchBegin { id, position },
            ffi::AppEventKind::TouchMove => Event::TouchMove { id, position },
            ffi::AppEventKind::TouchEnd => Event::TouchMove { id, position },
        };
        self.cx.process(&my_ui(), &rui_event, &mut self.vger.as_mut().unwrap())
    }
}

#[swift_bridge::bridge]
mod ffi {

    pub enum AppEventKind {
        TouchBegin,
        TouchMove,
        TouchEnd
    }

    #[swift_bridge(swift_repr = "struct")]
    pub struct AppEvent {
        x: f32,
        y: f32,
        id: usize,
        kind: AppEventKind
    }

    extern "Rust" {
        type AppState;

        #[swift_bridge(init)]
        fn new() -> AppState;

        fn setup_surface(&mut self, ca_layer_ptr: *mut c_void);
        fn update(&mut self, width: f32, height: f32);
        fn render(&mut self, width: f32, height: f32, scale: f32);
        fn process(&mut self, event: AppEvent);
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
