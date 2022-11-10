use raw_window_handle::{HasRawWindowHandle, HasRawDisplayHandle};
use wgpu::util::DeviceExt;
use crate::engine::renderer::{RendererTrait, Shader, ComputePipeline, BufferUsage, Buffer};

struct InternalComputePipeline {
    pipeline: wgpu::ComputePipeline,
    bind_groups: Vec<(usize, wgpu::BindGroup)>,
}

impl InternalComputePipeline {
    pub fn new(pipeline: wgpu::ComputePipeline) -> Self {
        Self {
            pipeline,
            bind_groups: Vec::new(),
        }
    }
}

pub struct WGPURenderer {
    surface : wgpu::Surface,
    device  : wgpu::Device,
    queue   : wgpu::Queue,
    config  : wgpu::SurfaceConfiguration,
    size    : winit::dpi::PhysicalSize<u32>,

    main_encoder: Option<wgpu::CommandEncoder>,
    main_surface_texture: Option<wgpu::SurfaceTexture>,
    main_texture_view: Option<wgpu::TextureView>,

    render_texture: wgpu::Texture,

    blit_pipeline: wgpu::RenderPipeline,
    blit_bind_group: wgpu::BindGroup,

    shaders : Vec<wgpu::ShaderModule>,
    compute_pipelines: Vec<InternalComputePipeline>,
    buffers : Vec<wgpu::Buffer>,
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

        let render_texture = device.create_texture(&wgpu::TextureDescriptor {
            label           : Some("RenderTexture"),
            size            : wgpu::Extent3d { width: size.width, height: size.height, depth_or_array_layers: 1 },
            mip_level_count : 1,
            sample_count    : 1,
            dimension       : wgpu::TextureDimension::D2,
            format          : wgpu::TextureFormat::Rgba8Unorm,
            usage           : wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        let render_texture_sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("RenderTextureSampler"),
            address_mode_u: wgpu::AddressMode::Repeat,
            address_mode_v: wgpu::AddressMode::Repeat,
            address_mode_w: wgpu::AddressMode::Repeat,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });

        let blit_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Blit Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../../shaders/fsq.wgsl").into())
        });

        let blit_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Blit Render Pipeline"),
            layout: None,
            vertex: wgpu::VertexState { module: &blit_shader, entry_point: "vs_main", buffers: &[] },
            primitive: wgpu::PrimitiveState {
                topology            : wgpu::PrimitiveTopology::TriangleStrip,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module      : &blit_shader,
                entry_point : "fs_main",
                targets     : &[Some(wgpu::ColorTargetState {
                    format      : config.format,
                    blend       : Some(wgpu::BlendState::REPLACE),
                    write_mask  : wgpu::ColorWrites::ALL,
                })],
            }),

            multiview: None,
        });

        let render_texture_view = render_texture.create_view(&wgpu::TextureViewDescriptor::default());

        let blit_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Blit Bind Group"),
            layout: &blit_pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&render_texture_view),
                },

                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&render_texture_sampler),
                },
            ]
        });

        Self {
            surface,
            device,
            queue,
            config,
            size,

            main_surface_texture: None,
            main_texture_view: None,
            main_encoder: None,

            render_texture,

            blit_pipeline,
            blit_bind_group,

            shaders: Vec::new(),
            compute_pipelines: Vec::new(),
            buffers: Vec::new(),
        }
    }

    fn get_size(&self) -> (u32, u32) {
        (self.config.width, self.config.height)
    }

    fn render_begin(&mut self) {
        let output = self.surface.get_current_texture().unwrap();
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        self.main_surface_texture = Some(output);
        self.main_texture_view = Some(view);
        self.main_encoder = Some(encoder);
    }

    fn render(&mut self) {
        let encoder = self.main_encoder.as_mut().unwrap();
        
        {
            let render_texture_view =self.render_texture.create_view(&wgpu::TextureViewDescriptor::default());
             
            let mut _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    Some(wgpu::RenderPassColorAttachment {
                        view: &render_texture_view,
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
                    })
                ],
                depth_stencil_attachment: None,
            });
        }
        
    }

    fn render_end(&mut self) {
        let mut encoder = self.main_encoder.take().unwrap();
        let view = self.main_surface_texture.as_ref().unwrap().texture.create_view(&wgpu::TextureViewDescriptor::default());

        {
            let mut post_process_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Post Process Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            });

            post_process_pass.set_pipeline(&self.blit_pipeline);
            post_process_pass.set_bind_group(0, &self.blit_bind_group, &[]);
            post_process_pass.draw(0..4, 0..1);
        }

        let output = self.main_surface_texture.take().unwrap();
        
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

        let id = self.shaders.len();
        self.shaders.push(module);

        Shader { id }
    }

    fn create_compute_pipeline(&mut self, shader: Shader, entry_point: Option<&'static str>) -> ComputePipeline {
        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label       : None,
            layout      : None,
            module      : &self.shaders[shader.id],
            entry_point : entry_point.unwrap_or("cs_main"),
        });

        let id = self.compute_pipelines.len();
        self.compute_pipelines.push(InternalComputePipeline::new(pipeline));

        ComputePipeline { id }
    }

    fn dispatch_post_process_compute_pipeline(&mut self, pipeline: ComputePipeline, workgroups: (u32, u32, u32)) {
        let encoder = self.main_encoder.as_mut().unwrap();

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.compute_pipelines[pipeline.id].pipeline.get_bind_group_layout(0),
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(
                        &self.render_texture.create_view(&wgpu::TextureViewDescriptor::default())
                    ),
                }
            ]
        });

        {
            let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                label: None,
            });

            pass.set_bind_group(0, &bind_group, &[]);

            for (group, bind_group) in &self.compute_pipelines[pipeline.id].bind_groups {
                let group = *group;
                
                if group <= 0 {
                    continue;
                }

                pass.set_bind_group(group as u32, bind_group, &[]);
            }

            pass.set_pipeline(&self.compute_pipelines[pipeline.id].pipeline);

            let (x, y, z) = workgroups;
            pass.dispatch_workgroups(self.config.width / x, self.config.height / y, z);
        }
    }

    fn create_buffer(&mut self, size: u64, usage: BufferUsage, read_only: bool) -> Buffer {
        let mut usage = match usage {
            BufferUsage::UNIFORM => wgpu::BufferUsages::UNIFORM,
            BufferUsage::STORAGE => wgpu::BufferUsages::STORAGE,
        };

        if read_only {
            usage = usage | wgpu::BufferUsages::COPY_DST;
        }

        let id = self.buffers.len();

        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size,
            usage,
            mapped_at_creation: false,
        });

        self.buffers.push(buffer);

        Buffer { id }
    }

    fn create_buffer_with_data<T: bytemuck::Pod>(&mut self, data: &T, usage: BufferUsage, read_only: bool) -> Buffer {
        let mut usage = match usage {
            BufferUsage::UNIFORM => wgpu::BufferUsages::UNIFORM,
            BufferUsage::STORAGE => wgpu::BufferUsages::STORAGE,
        };

        if read_only {
            usage = usage | wgpu::BufferUsages::COPY_DST;
        }

        let id = self.buffers.len();

        let buffer = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(data),
            usage,
        });

        self.buffers.push(buffer);

        Buffer { id }
    }

    fn update_buffer<T: bytemuck::Pod>(&self, buffer: Buffer, data: &T, offset: u64) {
        let data = bytemuck::bytes_of(data);
        self.queue.write_buffer(&self.buffers[buffer.id], offset, data);
    }

    fn destroy_buffer(&mut self, buffer: Buffer) {
        self.buffers[buffer.id].destroy();
    }

    fn set_binding_data(&mut self, pipeline: ComputePipeline, group: u32, data: &[Buffer]) {
        let pipeline = &mut self.compute_pipelines[pipeline.id];

        let entries = data.iter().enumerate().map(|(index, buff)| {
            wgpu::BindGroupEntry {
                binding: index as u32,
                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                    buffer: &self.buffers[buff.id],
                    offset: 0,
                    size: None,
                }),
            }
        });

        let bind_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &pipeline.pipeline.get_bind_group_layout(group),
            entries: &entries.collect::<Vec<_>>(),
        });

        let bind_group = (group as usize, bind_group);

        if pipeline.bind_groups.iter_mut().find(|(id, _)| *id == (group as usize)).is_some() {
            pipeline.bind_groups[group as usize]= bind_group;
        } else {
            pipeline.bind_groups.push(bind_group);
        }
    }
}