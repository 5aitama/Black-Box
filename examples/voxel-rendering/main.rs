pub mod camera;

use std::sync::{Arc, Mutex};

use camera::Camera;
use voxel_engine::engine::{
    renderer::{RendererTrait, BufferUsage},
    renderers::wgpu_renderer::WGPURenderer, 
    Engine
};

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Default)]
pub struct UniformScreenData {
    width   : f32,
    height  : f32,
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Default)]
pub struct UniformTimeData {
    time        : f32,
    delta_time  : f32,
}

#[repr(C)]
#[derive(bytemuck::Pod, bytemuck::Zeroable, Clone, Copy, Default)]
pub struct InData {
    screen_data: UniformScreenData,
    time_data: UniformTimeData,
    inv_proj_view_matrix: nalgebra::Matrix4<f32>,
    near: f32,
    far: f32,
    
    // Extra padding of 12bytes to align with
    // GPU memory...
    _padding: [u8; 12],
}

impl InData {
    pub fn new(size: (u32, u32), near: f32, far: f32) -> Self {
        let width   = size.0 as f32;
        let height  = size.1 as f32;

        let mut camera = Camera::new(width, height, 0.1, 1000.0, 45.0);
        camera.translate(&nalgebra_glm::vec3(1f32, 1.0, -15.0));

        Self {
            screen_data: UniformScreenData { width, height },
            time_data: UniformTimeData::default(),
            inv_proj_view_matrix: camera.get_proj_view_matrix().try_inverse().unwrap(),
            near,
            far,
            ..Default::default()
        }
    }

    pub fn update_proj_view_matrix(&mut self, camera: &Camera) {
        self.inv_proj_view_matrix = camera.get_proj_view_matrix().try_inverse().unwrap();
    }

    pub fn add_delta(&mut self, d: f32) {
        self.time_data.time += d;
    }
}

fn main() {
    let mut engine = Engine::<WGPURenderer>::new();

    let camera = engine.with_renderer_ref(|renderer| {
        let near = 0.1f32;
        let far  = 1000.0f32;
        let (width, height) = renderer.get_size();

        let mut camera = Camera::new(width as f32, height as f32, near, far, 45.0);
        camera.translate(&nalgebra_glm::vec3(1f32, 1.0, -15.0));

        Arc::new(Mutex::new(camera))
    });

    // Used to calculate time and delta time.
    let mut last_time = std::time::Instant::now();

    // Initialize our uniform data.
    let mut uniform_data = engine.with_renderer_ref(|renderer| {
        let camera = camera.lock().unwrap();
        InData::new(renderer.get_size(), camera.get_near(), camera.get_far())
    });

    // Create the uniform buffer and the compute pipeline.
    let (uniform_buffer, pipeline) = engine.with_renderer_mut(|renderer| {
        // Read and compile the wgsl shader.
        let shader = renderer.compile_shader(include_str!("shaders/test.wgsl"));

        // Create the compute pipeline that will use the shader
        // created above.
        let pipeline = renderer.create_compute_pipeline(shader,None);

        // Create the uniform buffer that will use the pipeline
        // created above.
        let uniform_buffer = renderer.create_buffer_with_data(&uniform_data, BufferUsage::UNIFORM, true);
        renderer.set_binding_data(pipeline, 1, 0, uniform_buffer);

        (uniform_buffer, pipeline)
    });

    let camera_one = camera.clone();
    let camera_two = camera.clone();

    engine.set_on_update_callback(move || {
        // This callback is called once each frame.
        // So in this callback you must put all your
        // game logic.
        
        let dt = last_time.elapsed();
        last_time = std::time::Instant::now();

        uniform_data.add_delta(dt.as_secs_f32());
        uniform_data.time_data.delta_time = dt.as_secs_f32();

        let mut camera = camera_one.lock().unwrap();
        let r = 2.0f32;
        let t = uniform_data.time_data.time;

        camera.set_position(&nalgebra_glm::Vec3::new(1f32 + t.cos() * r, 1f32 + t.sin() * r, -10.0));
    });

    engine.set_on_render_callback(move |renderer| {
        // This callback is called on each frame after
        // the update callback (so after the game logic)
        // and after rendering operations. So you
        // can interact with the renderer here.

        let camera = camera_two.lock().unwrap();
        uniform_data.inv_proj_view_matrix = camera.get_proj_view_matrix().try_inverse().unwrap();

        renderer.update_buffer(uniform_buffer, &uniform_data, 0);

        // Execute the compute shader each time we render a frame.
        renderer.dispatch_post_process_compute_pipeline(pipeline, (8, 8, 1));
    });

    // Run the engine.
    engine.run();
}