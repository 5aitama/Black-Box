pub struct Shader {
    pub(crate) module: wgpu::ShaderModule,
}

impl super::Renderer {
    /// Create and compile a [Shader].
    /// 
    /// # Arguments
    /// 
    /// * `source` - The wgsl shader source code.
    /// 
    pub fn create_shader(&self, source: &str) -> Shader {
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        Shader { module }
    }
}