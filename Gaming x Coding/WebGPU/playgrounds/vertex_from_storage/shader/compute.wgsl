struct Vertex {
    position: vec3<f32>,
    color: vec4<f32>,
}

@group(0) @binding(0) var<storage, read_write> vertices: array<Vertex>;


@compute
@workgroup_size(1)
fn cp_main(@builtin(global_invocation_id) global_invocation_id: vec3<u32>) {
    let x = global_invocation_id.x;

    var vertex: Vertex;
    if x % 3 == 0 {
        vertex.position = vec3(0.0, 0.0, 0.0);
        vertex.color = vec4(1.0, 0.0, 0.0, 1.0);
    } else if x % 3 == 1 {
        vertex.position = vec3(1.0, 0.0, 0.0);
        vertex.color = vec4(0.0, 1.0, 0.0, 1.0);
    } else {
        vertex.position = vec3(0.0, 1.0, 0.0);
        vertex.color = vec4(0.0, 0.0, 1.0, 1.0);
    }

    vertices[x] = vertex;
}