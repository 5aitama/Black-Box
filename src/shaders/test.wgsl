
@group(0) @binding(0)
var render_texture : texture_storage_2d<rgba8unorm, write>;

struct InData {
    screen_size: vec2<f32>,
    time: f32,
    delta_time: f32,
};

@group(1) @binding(0)
var<uniform> in_data: InData;

@compute
@workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // X coordinates normalized (0..1).
    let x = f32(id.x) / in_data.screen_size.x;

    // Y coordinates normalized (0..1). Notice
    // that i reverse the y axis because it will
    // start from top left top bottom right by
    // default instead of bottom left to top right.
    let y = f32(u32(in_data.screen_size.y) - id.y) / in_data.screen_size.y;

    let z = (cos((x + y) / 2.0 + in_data.time) + 1.0) / 2.0;

    // Write into the texture.
    textureStore(render_texture, vec2<i32>(id.xy), vec4<f32>(x, y, z, 1.0));
}