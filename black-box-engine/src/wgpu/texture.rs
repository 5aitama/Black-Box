pub struct FrameBuffer {
    texture: wgpu::Texture,
}

impl super::Renderer {
    /// Create a new [FrameBuffer].
    pub fn create_frame_buffer(&self) -> FrameBuffer {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label           : Some("FrameBuffer"),
            size            : wgpu::Extent3d { width: self.size.width, height: self.size.height, depth_or_array_layers: 1 },
            mip_level_count : 1,
            sample_count    : 1,
            dimension       : wgpu::TextureDimension::D2,
            format          : wgpu::TextureFormat::Rgba8Unorm,
            usage           : wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        FrameBuffer { texture }
    }

    /// Update a render texture. This will update the size
    /// of the render texture to match with the current render
    /// size.
    /// 
    /// Don't call this method every frame to avoid unecessary
    /// texture re-creation, call it only when the render texture
    /// (the window) will be resized.
    /// 
    /// # Arguments
    /// 
    /// * `frame_buffer` - The [FrameBuffer] to update.
    /// 
    pub fn update_frame_buffer(&self, frame_buffer: &mut FrameBuffer) {
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label           : Some("FrameBuffer"),
            size            : wgpu::Extent3d { width: self.size.width, height: self.size.height, depth_or_array_layers: 1 },
            mip_level_count : 1,
            sample_count    : 1,
            dimension       : wgpu::TextureDimension::D2,
            format          : wgpu::TextureFormat::Rgba8Unorm,
            usage           : wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        });

        frame_buffer.texture.destroy();
        frame_buffer.texture = texture;
    }
}