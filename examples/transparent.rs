use bevy::{
    prelude::{
        shape, AlphaMode, App, Assets, Camera3dBundle, ClearColor, Color, Commands,
        MaterialMeshBundle, Mesh, ResMut, Transform, Vec3, Startup,
    },
    DefaultPlugins,
};
use bevy_normal_material::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NormalMaterialPlugin)
        .insert_resource(ClearColor(Color::rgb(0.01, 0.02, 0.08)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NormalMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        transform: Transform::from_xyz(0.0, 0.25, 0.0),
        material: materials.add(NormalMaterial {
            opacity: 0.5,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..Default::default()
        }),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });
}
