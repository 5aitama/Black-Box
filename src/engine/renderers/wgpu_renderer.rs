use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};

use crate::engine::renderer::{RendererTrait, Shader};

pub struct WGPURenderer {
    surface : wgpu::Surface,
    device  : wgpu::Device,
    queue   : wgpu::Queue,
    config  : wgpu::SurfaceConfiguration,
    size    : winit::dpi::PhysicalSize<u32>,

    shaders : Vec<wgpu::ShaderModule>,
}

impl RendererTrait for WGPURenderer {
    fn new(surface: &(impl HasRawWindowHandle + HasRawDisplayHandle), size: (u32, u32)) -> Self where Self: Sized {
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

            shaders: Vec::new(),
        }
    }

    fn render(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.01,
                            g: 0.01,
                            b: 0.01,
                            a: 1.00,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });
        }
    
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();
    }

    fn resize(&mut self, new_size: (u32, u32)) {
        let new_size = winit::dpi::PhysicalSize::new(new_size.0, new_size.1);

        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn compile_shader(&mut self, source: impl Into<String>) -> Shader {

        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source.into().into()),
        });

        let id = self.shaders.len() as u32;
        self.shaders.push(module);

        Shader { id }
    }
}