@group(0) @binding(0)
var render_texture : texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    // X coordinates normalized (0..1).
    let x = f32(id.x) / 1600.0;

    // Y coordinates normalized (0..1). Notice
    // that i reverse the y axis because it will
    // start from top left top bottom right by
    // default instead of bottom left to top right.
    let y = f32(u32(1200) - id.y) / 1200.0;

    // Write into the texture.
    textureStore(render_texture, vec2<i32>(id.xy), vec4<f32>(x, y, (x + y) / 2.0, 1.0));
}