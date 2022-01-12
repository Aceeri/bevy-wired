use bevy::{core_pipeline::AlphaMask3d, prelude::*};
use bevy_stylized_wireframe::*;

/// This example shows how to manually render 2d items using "mid level render apis" with a custom pipeline for 2d meshes
/// It doesn't use the [`Material2d`] abstraction, but changes the vertex buffer to include vertex color
/// Check out the "mesh2d" example for simpler / higher level 2d meshes
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleWireframePlugin)
        .insert_resource(Msaa { samples: 4 })
        //var mainStroke = mix(fill, stroke, edge);
        //var outColor = vec4<f32>(0.0)
        //return vec4<f32>(edge, 0.0, 0.0, 1.0);
        //return vec4<f32>(0.0, 0.0, 0.0, 0.5);
        .insert_resource(ClearColor(Color::rgba_u8(66, 135, 245, 255)))
        .add_startup_system(cube)
        .insert_resource(Accumulate(0.0))
        .add_system(rotate_camera)
        .run();
}

pub fn cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut cube = Mesh::from(bevy::render::mesh::shape::Icosphere::default());
    cube.compute_barycentric();

    // We can now spawn the entities for the star and the camera
    commands.spawn_bundle((
        SimpleWireframe::default(),
        meshes.add(cube),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
        AlphaMode::Blend,
    ));

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(-1.0, 2.5, -1.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}

pub struct Accumulate(f32);

pub fn rotate_camera(
    mut accumulate: ResMut<Accumulate>,
    time: Res<Time>,
    mut cameras: Query<(&mut Transform, &Camera)>,
) {
    accumulate.0 += time.delta_seconds() / 5.0;
    let scale = 5.0;
    for (mut transform, cam) in cameras.iter_mut() {
        *transform =
            Transform::from_xyz(accumulate.0.cos() * scale, 0.5, accumulate.0.sin() * scale)
                .looking_at(Vec3::ZERO, Vec3::Y);
    }
}
