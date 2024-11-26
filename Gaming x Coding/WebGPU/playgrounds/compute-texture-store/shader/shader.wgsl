
@group(0) @binding(0) var texture: texture_storage_2d<rgba8unorm, write>;

@compute
@workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
    let x = global_invocation_id.x;
    let y = global_invocation_id.y;

    textureStore(texture, vec2(x, y), vec4(1.0, 0.0, 0.0, 1.0));
}