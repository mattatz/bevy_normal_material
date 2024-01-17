use bevy::{
    asset::load_internal_asset,
    prelude::{MaterialPlugin, Plugin, Shader},
};

use crate::{prelude::NormalMaterial, SHADER_HANDLE};

pub struct NormalMaterialPlugin;

impl Plugin for NormalMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        load_internal_asset!(
            app,
            SHADER_HANDLE,
            "./shaders/normal.wgsl",
            Shader::from_wgsl
        );
        app.add_plugins(MaterialPlugin::<NormalMaterial>::default());
    }
}
