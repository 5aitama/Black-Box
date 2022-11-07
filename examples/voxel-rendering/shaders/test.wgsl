
@group(0) @binding(0)
var render_texture : texture_storage_2d<rgba8unorm, write>;

struct InData {
    screen_size: vec2<f32>,
    time: f32,
    delta_time: f32,
    inv_proj_view_matrix: mat4x4<f32>,
    near: f32,
    far: f32,
};

@group(1) @binding(0)
var<uniform> in_data: InData;

let MAX_RAY_STEPS: i32 = 128;

@compute
@workgroup_size(8, 8)
fn cs_main(@builtin(global_invocation_id) id: vec3<u32>) {
    var uv: vec2<f32> = (vec2<f32>(id.xy) / in_data.screen_size) * 2.0 - 1.0;
    uv.y = 0.0 - uv.y;

    let ray_pos: vec3<f32> = (in_data.inv_proj_view_matrix * vec4<f32>(uv, 2.0, 1.0) * in_data.near).xyz;
    let ray_dir: vec3<f32> = (in_data.inv_proj_view_matrix * vec4<f32>(uv * (in_data.far - in_data.near), in_data.far + in_data.near, in_data.far - in_data.near)).xyz;

    let ray_step: vec3<i32> = vec3<i32>(sign(ray_dir));
    let delta_dist: vec3<f32> = 1.0 / abs(ray_dir);

    var map_pos: vec3<i32> = vec3<i32>(floor(ray_pos + 0.));
    var side_dist: vec3<f32> = (sign(ray_dir) * (vec3<f32>(map_pos) - ray_pos) + (sign(ray_dir) * 0.5) + 0.5) * delta_dist;
    var color: vec3<f32> = vec3<f32>(0.0, 0.0, 0.0);
    var mask: vec3<i32> = vec3<i32>(0, 0, 0);

    for(var i: i32 = 0; i < MAX_RAY_STEPS; i++) {
        if (
            (map_pos.x ==  2 && map_pos.y ==  0 && map_pos.z == 0) ||
            (map_pos.x == -2 && map_pos.y ==  0 && map_pos.z == 0) ||
            (map_pos.x ==  0 && map_pos.y ==  2 && map_pos.z == 0) ||
            (map_pos.x ==  0 && map_pos.y == -2 && map_pos.z == 0) ||
            (map_pos.x ==  0 && map_pos.y ==  0 && map_pos.z == 0)
        )
        {
            let d: f32 = length(vec3<f32>(mask) * (side_dist - delta_dist));
            let dst: vec3<f32> = ray_pos + ray_dir * d;
            
            color = (dst - vec3<f32>(map_pos));

            // if (color.z <= 0.000001) {
            //     color = vec3<f32>(1.0, 0.0, 0.0);
            // }

            // if (color.x <= 0.000001) {
            //     color = vec3<f32>(0.0, 1.0, 0.0);
            // }
            
            textureStore(render_texture, vec2<i32>(id.xy), vec4<f32>(color, 1.0));
            return;
        }

        if (side_dist.x <= side_dist.y) {
            if (side_dist.x <= side_dist.z) {
                side_dist.x += delta_dist.x;
                map_pos.x += ray_step.x;
                mask = vec3<i32>(1, 0, 0);
            } else {
                side_dist.z += delta_dist.z;
                map_pos.z += ray_step.z;
                mask = vec3<i32>(0, 0, 1);
            }
        } else {
            if (side_dist.y <= side_dist.z) {
                side_dist.y += delta_dist.y;
                map_pos.y += ray_step.y;
                mask = vec3<i32>(0, 1, 0);
            } else {
                side_dist.z += delta_dist.z;
                map_pos.z += ray_step.z;
                mask = vec3<i32>(0, 0, 1);
            }
        }
    }

    textureStore(render_texture, vec2<i32>(id.xy), vec4<f32>(color, 1.0));
}