use voxel_engine::engine::{
    renderer::RendererTrait,
    renderers::wgpu_renderer::WGPURenderer, 
    Engine
};

fn main() {
    // Create a new instance of the engine with a renderer
    // based on WGPU.
    let mut engine = Engine::<WGPURenderer>::new();

    // Create a compute pipeline wich execute a compute shader.
    //
    // I call the unwrap() at the end because i'm sure that it
    // will never panic but for a real use we need always to
    // check it...
    let pipeline = engine.with_renderer_mut(|renderer| {
        let shader_path = std::path::Path::new("src/shaders/test.wgsl");
        
        // Read and compile the wgsl shader.
        let shader = match std::fs::read_to_string(shader_path) {
            Ok(source) => Some(renderer.compile_shader(source)),
            _ => None
        };

        // Create the compute pipeline from the shader created above.
        // Notice that i'm doing the shader compilation and the pipeline
        // creation together and i'm return only the pipeline because i don't
        // need the shader after that.
        match shader {
            Some(shader) => Some(renderer.create_compute_pipeline(shader,None)),
            _ => None,
        }
    }).unwrap();


    engine.set_on_update_callback(|| {
        // This callback is called once each frame.
        // So in this callback you must put all your
        // game logic.
    });

    engine.set_on_render_callback(move |renderer| {
        // This callback is called on each frame after
        // the update callback (so after the game logic)
        // and after rendering operations. So you
        // can interact with the renderer here.

        // Execute the compute shader each time we render
        // a frame.
        renderer.dispatch_post_process_compute_pipeline(pipeline, (8, 8, 0));
    });

    // Run the engine.
    engine.run();
}