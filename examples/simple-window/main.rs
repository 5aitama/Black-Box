use voxel_engine::engine::{Engine, renderers::wgpu_renderer::WGPURenderer};

fn main() {
    let engine = Engine::<WGPURenderer>::new();
    engine.run();
}