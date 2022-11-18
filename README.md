
# Bevy Normal Material

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
        .add_plugin(NormalMaterialPlugin);
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
