struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
}

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
}

@group(0) @binding(0) var<storage> vertices: array<Vertex>;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let x = f32(vertex_index % 128) / 64.0;
    let y = f32(vertex_index / 128) / 64.0;


    var out: VertexOutput;

    // out.position = vec4(vertices[vertex_index].position, 1.0);
    out.position = vec4(x, y, 0.0, 1.0);
    out.color = vertices[vertex_index].color;
    // if vertex_index == 0 {
    //     out.color = vec4(1.0, 0.0, 0.0, 1.0);
    // } else if vertex_index == 1 {
    //     out.color = vec4(0.0, 1.0, 0.0, 1.0);
    // } else if vertex_index == 2 {
    //     out.color = vec4(0.0, 0.0, 1.0, 1.0);
    // } else if vertex_index == 3 {
    //     out.color = vec4(1.0, 1.0, 1.0, 1.0);
    // }
    // out.color = vec4(vertices[vertex_index].position, 1.0);

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

