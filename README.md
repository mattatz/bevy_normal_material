
# Bevy Normal Material

[![crates.io](https://img.shields.io/crates/v/bevy_normal_material)](https://crates.io/crates/bevy_normal_material)

Simple normal material for Bevy.

![Example](https://user-images.githubusercontent.com/1085910/202606799-2f8851cf-3006-44f2-a0e5-5cccbd76ea3a.png)

## Usage

### System setup

Add the plugin to your app:

```rust
use bevy::prelude::*;
use bevy_normal_material::prelude::*;

fn main() {
    App::new()
        .add_plugins(NormalMaterialPlugin);
}
```

### Apply a component to a MaterialMeshBundle

```rust
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<NormalMaterial>>,
) {
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(NormalMaterial::default()),
        ..Default::default()
    });
}
```

## Compatibility

| bevy | bevy_normal_material |
| ---- | ------------- |
| 0.9  | 0.1           |
| 0.10  | 0.2           |
| 0.11  | 0.3           |
| 0.12  | 0.4           |
| 0.13  | 0.5           |
| 0.14  | 0.6           |
| 0.15  | 0.7           |
| 0.16  | 0.8           |