use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

/// Represent a shader.
pub struct Shader {
    /// The shader id that must be unique. It will be
    /// used by renderer to identify wich shader is.
    pub(crate) id: u32,
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

    /// Render a frame.
    fn render(&mut self);

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

}