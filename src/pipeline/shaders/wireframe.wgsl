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
    var position_along = max(in.barycentric.x, in.barycentric.y);
    if (in.barycentric.y < in.barycentric.x && in.barycentric.y < in.barycentric.z) {
        position_along = 1.0 - position_along;
    }

    var dash_repeats = 10.0;
    var dash_length = 0.1;
    var offset = 1.0 / dash_repeats * dash_length / 2.0;
    var offset = offset + (1.0 / dash_repeats / 2.0);
    var pattern = fract((position_along + offset) * dash_repeats);

    var computed_thickness = 0.1;
    var squeeze_min = 1.0;
    var squeeze_max = 1.0;
    var computed_thickness = computed_thickness * mix(squeeze_min, squeeze_max, (1.0 - sin(position_along * 6.283)));
    var computed_thickness = computed_thickness * (1.0 - aastep(dash_length, pattern));
    var computed_thickness = 1.0 - aastep(dash_length, pattern);

    var barycentric_distance = min(min(in.barycentric.x, in.barycentric.y), in.barycentric.z);
    var edge = 1.0 - aastep(computed_thickness, barycentric_distance);

    var stroke = vec3<f32>(0.059, 0.059, 0.059);
    var out_color = vec4<f32>(stroke, edge);
    if (!is_front) {
        var fill = vec3<f32>(0.702, 0.702, 0.702);
        out_color = vec4<f32>(fill, edge);
    } 

    return out_color;
}
