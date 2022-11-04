use voxel_engine::engine::{
    renderer::RendererTrait,
    renderers::wgpu_renderer::WGPURenderer, 
    Engine
};

fn main() {
    // Create a new instance of the engine with a renderer
    // based on WGPU.
    let mut engine = Engine::<WGPURenderer>::new();

    let shader = engine.with_renderer_mut(|renderer| {
        let shader_path = std::path::Path::new("src/shaders/test.wgsl");
        
        match std::fs::read_to_string(shader_path) {
            Ok(source) => Some(renderer.compile_shader(source)),
            _ => None
        }
    });

    println!("Shader is compiled ? {} !", if shader.is_some() { "Yes" } else { "No" } );

    engine.set_on_update_callback(|| {
        // This callback is called once each frame.
        // So in this callback you must put all your
        // game logic.
    });

    engine.set_on_render_callback(|_renderer| {
        // This callback is called on each frame after
        // the update callback (so after the game logic)
        // and after rendering operations. So you
        // can interact with the renderer here.
    });

    // Run the engine.
    engine.run();
}