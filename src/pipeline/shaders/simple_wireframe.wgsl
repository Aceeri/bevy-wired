
#import bevy_pbr::mesh_view_bind_group
#import bevy_pbr::mesh_struct

struct Vertex {
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] normal: vec3<f32>;
    [[location(2)]] uv: vec2<f32>;
    [[location(3)]] barycentric: vec3<f32>;
};

[[group(1), binding(0)]]
var<uniform> mesh: Mesh;

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] position: vec3<f32>;
    [[location(1)]] uv: vec2<f32>;
    [[location(2)]] barycentric: vec3<f32>;
};

[[stage(vertex)]]
fn vertex(vertex: Vertex) -> VertexOutput {
    let world_position = mesh.model * vec4<f32>(vertex.position, 1.0);

    var out: VertexOutput;
    out.position = vertex.position;
    out.barycentric = vertex.barycentric;
    out.uv = vertex.uv;
    out.clip_position = view.view_proj * world_position;

    return out;
}

fn aastep(threshold: f32, distance: f32) -> f32 {
    var afwidth = fwidth(distance) * 0.5;
    return smoothStep(threshold - afwidth, threshold + afwidth, distance);
}

[[stage(fragment)]]
fn fragment(
    [[builtin(front_facing)]] is_front: bool,
    in: VertexOutput
) -> [[location(0)]] vec4<f32> {
    var edge = 1.0;
    var fill = vec3<f32>(0.902, 0.902, 0.902);
    var stroke = vec3<f32>(0.059, 0.059, 0.059);
    var out_color = vec4<f32>(stroke, edge);
    if (!is_front) {
        out_color = vec4<f32>(fill, edge);
    } 

    return out_color;
}
