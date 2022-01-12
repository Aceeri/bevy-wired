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

// // This function returns the fragment color for our styled wireframe effect
// // based on the barycentric coordinates for this fragment
// fn getStyledWireframe (vec3 barycentric) -> vec4<f32> {
//   // this will be our signed distance for the wireframe edge
//   float d = min(min(barycentric.x, barycentric.y), barycentric.z);

//   // we can modify the distance field to create interesting effects & masking
//   float noiseOff = 0.0;
//   if (noiseA) noiseOff += noise(vec4(vPosition.xyz * 1.0, time * 0.35)) * 0.15;
//   if (noiseB) noiseOff += noise(vec4(vPosition.xyz * 80.0, time * 0.5)) * 0.12;
//   d += noiseOff;

//   // for dashed rendering, we can use this to get the 0 .. 1 value of the line length
//   float positionAlong = max(barycentric.x, barycentric.y);
//   if (barycentric.y < barycentric.x && barycentric.y < barycentric.z) {
//     positionAlong = 1.0 - positionAlong;
//   }

//   // the thickness of the stroke
//   float computedThickness = thickness;

//   // if we want to shrink the thickness toward the center of the line segment
//   if (squeeze) {
//     computedThickness *= mix(squeezeMin, squeezeMax, (1.0 - sin(positionAlong * PI)));
//   }

//   // if we should create a dash pattern
//   if (dashEnabled) {
//     // here we offset the stroke position depending on whether it
//     // should overlap or not
//     float offset = 1.0 / dashRepeats * dashLength / 2.0;
//     if (!dashOverlap) {
//       offset += 1.0 / dashRepeats / 2.0;
//     }

//     // if we should animate the dash or not
//     if (dashAnimate) {
//       offset += time * 0.22;
//     }

//     // create the repeating dash pattern
//     float pattern = fract((positionAlong + offset) * dashRepeats);
//     computedThickness *= 1.0 - aastep(dashLength, pattern);
//   }

//   // compute the anti-aliased stroke edge  
//   float edge = 1.0 - aastep(computedThickness, d);

//   // now compute the final color of the mesh
//   vec4 outColor = vec4(0.0);
//   if (seeThrough) {
//     outColor = vec4(stroke, edge);
//     if (insideAltColor && !gl_FrontFacing) {
//       outColor.rgb = fill;
//     }
//   } else {
//     vec3 mainStroke = mix(fill, stroke, edge);
//     outColor.a = 1.0;
//     if (dualStroke) {
//       float inner = 1.0 - aastep(secondThickness, d);
//       vec3 wireColor = mix(fill, stroke, abs(inner - edge));
//       outColor.rgb = wireColor;
//     } else {
//       outColor.rgb = mainStroke;
//     }
//   }

//   return outColor;
// }

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
        var position_along = 1.0 - position_along;
    }

    var dash_repeats = 8.0;
    var dash_length = 0.5;
    var offset = 1.0 / dash_repeats * dash_length / 2.0;
    var offset = offset + (1.0 / dash_repeats / 2.0);
    var pattern = fract((position_along + offset) * dash_repeats);

    var computed_thickness = 0.1;
    var squeeze_min = 0.0;
    var squeeze_max = 0.7;
    var computed_thickness = computed_thickness * mix(squeeze_min, squeeze_max, (1.0 - sin(position_along * 3.1415926535)));
    var computed_thickness = computed_thickness * (1.0 - aastep(dash_length, pattern));

    var barycentric_distance = min(min(in.barycentric.x, in.barycentric.y), in.barycentric.z);
    var edge = 1.0 - aastep(computed_thickness, barycentric_distance);


    if (edge <= 0.2) {
        discard;
    }

    if (is_front) {
        var stroke = vec3<f32>(0.0, 0.0, 0.0);
        return vec4<f32>(stroke, edge);
    } else {
        var stroke = vec3<f32>(0.2, 0.2, 0.2);
        return vec4<f32>(stroke, edge);
    }
    //var mainStroke = mix(fill, stroke, edge);
    //var outColor = vec4<f32>(0.0)
    //return vec4<f32>(edge, 0.0, 0.0, 1.0);
    //return vec4<f32>(0.0, 0.0, 0.0, 0.5);
}
