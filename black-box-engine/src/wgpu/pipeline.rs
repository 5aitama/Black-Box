pub struct RenderPipeline {
    pub(crate) pipeline: wgpu::RenderPipeline,
}

pub struct ComputePipeline {
    pub(crate) pipeline: wgpu::ComputePipeline,
}

impl super::Renderer {
    /// Create a render pipeline.
    /// 
    /// # Arguments
    /// 
    /// * `shader` - The shader used by the render pipeline.
    /// 
    pub fn create_render_pipeline(&self, shader: &super::shader::Shader) -> RenderPipeline {
        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            primitive: wgpu::PrimitiveState {
                topology            : wgpu::PrimitiveTopology::TriangleList,
                strip_index_format  : Some(wgpu::IndexFormat::Uint32),
                front_face          : wgpu::FrontFace::Cw,
                cull_mode           : Some(wgpu::Face::Back),
                unclipped_depth     : false,
                polygon_mode        : wgpu::PolygonMode::Fill,
                conservative        : false,
            },
            vertex: wgpu::VertexState {
                module      : &shader.module,
                entry_point : "vs_main",
                buffers     : &[],
            },
            fragment: Some(wgpu::FragmentState {
                module      : &shader.module,
                entry_point : "fs_main",
                targets     : &[
                    Some(wgpu::ColorTargetState {
                        format      : self.config.format,
                        blend       : Some(wgpu::BlendState::ALPHA_BLENDING),
                        write_mask  : wgpu::ColorWrites::ALL,
                    }),
                ]
            }),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count   : 1,
                mask    : !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        RenderPipeline { pipeline }
    }

    /// Create a compute pipeline.
    /// 
    /// # Arguments
    /// 
    /// * `shader` - The shader used by the compute pipeline.
    /// 
    pub fn create_compute_pipeline(&self, shader: &super::shader::Shader) -> ComputePipeline {
        let pipeline = self.device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: None,
            layout: None,
            module: &shader.module,
            entry_point: "cs_main",
        });

        ComputePipeline { pipeline }
    }
}