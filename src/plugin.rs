use bevy::prelude::{Assets, MaterialPlugin, Plugin, Shader};

use crate::{prelude::NormalMaterial, SHADER_HANDLE};

pub struct NormalMaterialPlugin;

impl Plugin for NormalMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut shaders = app.world.get_resource_mut::<Assets<Shader>>().unwrap();
        shaders.set_untracked(
            SHADER_HANDLE,
            Shader::from_wgsl(include_str!("./shaders/normal.wgsl")),
        );
        app.add_plugin(MaterialPlugin::<NormalMaterial>::default());
    }
}
