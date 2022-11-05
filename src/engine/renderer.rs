use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

/// Represent a shader.
#[derive(Clone, Copy)]
pub struct Shader {
    /// The shader id that must be unique. It will be
    /// used by renderer to identify wich shader is.
    pub(crate) id: usize,
}

/// Represente a compute pipeline.
#[derive(Clone, Copy)]
pub struct ComputePipeline {
    /// This is the unique id of the compute pipeline.
    /// This will be used internaly by the renderer to
    /// identify the compute pipeline.
    pub(crate) id: usize,
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



    /// Dispatch a compute pipeline.
    /// 
    /// # Arguments
    /// 
    /// * `pipeline` - The pipeline to dispatch.
    /// * `workgroups` - The amount of worker for each group.
    fn dispatch_post_process_compute_pipeline(&mut self, pipeline: ComputePipeline, workgroups: (u32, u32, u32));
}