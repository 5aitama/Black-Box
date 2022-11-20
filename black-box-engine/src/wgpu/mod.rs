pub mod texture;
pub mod shader;
pub mod pipeline;
pub mod pass;

use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};

pub struct Renderer {
    pub(crate) surface : wgpu::Surface,
    pub(crate) device  : wgpu::Device,
    pub(crate) queue   : wgpu::Queue,
    pub(crate) config  : wgpu::SurfaceConfiguration,
    pub(crate) size    : winit::dpi::PhysicalSize<u32>,
}

impl Renderer {
    pub fn new(surface: &(impl HasRawWindowHandle + HasRawDisplayHandle), size: (u32, u32)) -> Self where Self: Sized {
        let size = winit::dpi::PhysicalSize::new(size.0, size.1);
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(surface) };
        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference        : wgpu::PowerPreference::default(),
                compatible_surface      : Some(&surface),
                force_fallback_adapter  : false,
            },
        )).unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits  : wgpu::Limits::default(),
                label   : None,
            },
            None,
        )).unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage       : wgpu::TextureUsages::RENDER_ATTACHMENT,
            format      : surface.get_supported_formats(&adapter)[0],
            width       : size.width,
            height      : size.height,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode  : wgpu::CompositeAlphaMode::Auto,
        };

        surface.configure(&device, &config);

        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
}