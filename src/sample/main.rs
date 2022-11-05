use voxel_engine::engine::{
    renderer::{RendererTrait, BufferUsage},
    renderers::wgpu_renderer::WGPURenderer, 
    Engine
};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy)]
pub struct InData {
    screen_width: f32,
    screen_height: f32,
    time: f32,
    delta_time: f32,
}

impl InData {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            screen_width    : size.0 as f32,
            screen_height   : size.1 as f32,
            time            : 0.0,
            delta_time      : 0.0,
        }
    }
}

fn main() {
    let mut engine = Engine::<WGPURenderer>::new();

    // Used to calculate time and delta time.
    let mut last_time = std::time::Instant::now();

    // Initialize our uniform data.
    let mut uniform_data = engine.with_renderer_ref(|renderer| InData::new(renderer.get_size()));

    // Create the uniform buffer and the compute pipeline.
    let (uniform_buffer, pipeline) = engine.with_renderer_mut(|renderer| {
        let shader_path = std::path::Path::new("src/shaders/test.wgsl");
        
        // Read and compile the wgsl shader.
        let shader = match std::fs::read_to_string(shader_path) {
            Ok(source) => Some(renderer.compile_shader(source)),
            _ => None
        };

        // Create the compute pipeline that will use the shader
        // created above.
        let pipeline = match shader {
            Some(shader) => Some(renderer.create_compute_pipeline(shader,None)),
            _ => None,
        };

        // Create the uniform buffer that will use the pipeline
        // created above.
        let uniform_buffer = match pipeline {
            Some(pipeline) => {
                let buff = renderer.create_buffer_with_data(&uniform_data, BufferUsage::UNIFORM, true);

                renderer.set_binding_data(pipeline, 1, 0, buff);

                Some(buff)
            }
            _ => None,
        };

        (uniform_buffer, pipeline)
    });

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

        // Calculate the delta time and time and update
        // our uniform buffer with them.
        let dt = last_time.elapsed();
        last_time = std::time::Instant::now();

        uniform_data.time += dt.as_secs_f32();
        uniform_data.delta_time = dt.as_secs_f32();
        renderer.update_buffer(uniform_buffer.unwrap(), &uniform_data, 0);

        // Execute the compute shader each time we render a frame.
        renderer.dispatch_post_process_compute_pipeline(pipeline.unwrap(), (8, 8, 0));
    });

    // Run the engine.
    engine.run();
}