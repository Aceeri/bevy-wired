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
        .insert_resource(ClearColor(Color::rgba_u8(250, 250, 250, 255)))
        .add_startup_system(cube)
        .insert_resource(Accumulate(0.0))
        .add_system(rotate_camera)
        .run();
}

pub fn cube(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let mut mesh = Mesh::from(bevy::render::mesh::shape::Torus::default());
    mesh.compute_barycentric();

    commands.spawn_bundle((
        SimpleWireframe::default(),
        meshes.add(mesh),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
    ));

    let mut mesh = Mesh::from(bevy::render::mesh::shape::UVSphere {
        radius: 0.3,
        ..Default::default()
    });
    mesh.compute_barycentric();
    commands.spawn_bundle((
        SimpleWireframe::default(),
        meshes.add(mesh),
        Transform::default(),
        GlobalTransform::default(),
        Visibility::default(),
        ComputedVisibility::default(),
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
    mut cameras: Query<&mut Transform, With<Camera>>,
) {
    accumulate.0 += time.delta_seconds() / 5.0;
    let scale = 3.0;
    for mut transform in cameras.iter_mut() {
        *transform =
            Transform::from_xyz(accumulate.0.cos() * scale, (accumulate.0 / 1.5).cos() * scale, accumulate.0.sin() * scale)
                .looking_at(Vec3::ZERO, Vec3::Y);
    }
}
