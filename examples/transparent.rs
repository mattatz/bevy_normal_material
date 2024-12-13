use bevy::pbr::MeshMaterial3d;
use bevy::prelude::{Camera3d, Mesh3d};
use bevy::{
    math::primitives::Cuboid,
    prelude::{
        AlphaMode, App, Assets, ClearColor, Color, Commands, Mesh, ResMut, Startup, Transform, Vec3,
    },
    DefaultPlugins,
};
use bevy_normal_material::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(NormalMaterialPlugin)
        .insert_resource(ClearColor(Color::srgb(0.01, 0.02, 0.08)))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NormalMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(NormalMaterial {
            opacity: 0.5,
            alpha_mode: AlphaMode::Blend,
            cull_mode: None,
            ..Default::default()
        })),
        Transform::from_xyz(0.0, 0.25, 0.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}
