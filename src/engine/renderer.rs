use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

/// Represent a shader.
#[derive(Clone, Copy)]
pub struct Shader {
    pub(crate) id: usize,
}

/// Represente a compute pipeline.
#[derive(Clone, Copy)]
pub struct ComputePipeline {
    pub(crate) id: usize,
}

/// Represent a compute buffer.
#[derive(Clone, Copy)]
pub struct Buffer {
    pub(crate) id: usize,
}

pub enum BufferUsage {
    /// Use the buffer as an uniform buffer (can be used in shader as binding)
    UNIFORM = 1,
    STORAGE = 2,
}

pub trait RendererTrait {
    /// Create a new Renderer from a surface that implement
    /// [raw_window_handle] traits.
    /// 
    /// # Arguments
    /// 
    /// * `surface` - The surface on wich the renderer will be render things.
    /// * `size` - The size of the surface in pixels.
    /// 
    fn new(surface: &(impl HasRawWindowHandle + HasRawDisplayHandle), size: (u32, u32)) -> Self where Self: Sized;

    fn render_begin(&mut self);

    /// Render a frame.
    fn render(&mut self);

    fn render_end(&mut self);

    /// Resize the renderer.
    /// 
    /// # Arguments
    /// 
    /// * `new_size` - The new renderer size in pixels.
    /// 
    fn resize(&mut self, new_size: (u32, u32));

    /// Get the renderer size.
    fn get_size(&self) -> (u32, u32);

    /// Compile a shader from source.
    /// 
    /// # Arguments
    /// 
    /// * `source` - The shader source code.
    fn compile_shader(&mut self, source: impl Into<String>) -> Shader;

    /// Create a new compute pipeline.
    /// 
    /// # Arguments
    /// 
    /// * `shader` - The shader used by the compute pipeline.
    /// * `entry_point` - The name of the entry point of the compute shader (by default is `"cs_main"`).
    fn create_compute_pipeline(&mut self, shader: Shader, entry_point: Option<&'static str>) -> ComputePipeline;

    /// Create a buffer.
    /// 
    /// # Arguments
    /// 
    /// * `size`        - The size of the buffer.
    /// * `usage`       - The usage(s) of the buffer.
    /// * `read_only`   - `true` if the buffer is read only otherwise `false`
    /// 
    fn create_buffer(&mut self, size: u64, usage: BufferUsage, read_only: bool) -> Buffer;

    /// Create a buffer and initialize it with some data.
    /// 
    /// # Arguments
    /// 
    /// * `data`        - The data to put into the buffer.
    /// * `usage`       - The usage(s) of the buffer.
    /// * `read_only`   - `true` if the buffer is read only otherwise `false`
    fn create_buffer_with_data<T: bytemuck::Pod>(&mut self, data: &T, usage: BufferUsage, read_only: bool) -> Buffer;

    /// Update the buffer data.
    /// 
    /// # Arguments
    /// 
    /// * `buffer`  - The buffer to update.
    /// * `data`    - The data to copy from.
    /// * `offset`  - The start index at where the data must be copied.
    /// 
    fn update_buffer<T: bytemuck::Pod>(&self, buffer: Buffer, data: &T, offset: u64);

    /// Destory a buffer.
    /// 
    /// # Arguments
    /// 
    /// * `buffer` - The buffer to destory.
    /// 
    fn destroy_buffer(&mut self, buffer: Buffer);

    /// Dispatch a compute pipeline.
    /// 
    /// # Arguments
    /// 
    /// * `pipeline` - The pipeline to dispatch.
    /// * `workgroups` - The amount of worker for each group.
    fn dispatch_post_process_compute_pipeline(&mut self, pipeline: ComputePipeline, workgroups: (u32, u32, u32));

    fn set_binding_data(&mut self, pipeline: ComputePipeline, group: u32, data: &[Buffer]);
}