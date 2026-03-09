use bevy::{asset::embedded_asset, prelude::MaterialPlugin};

use crate::prelude::NormalMaterial;

pub struct NormalMaterialPlugin;

impl bevy::prelude::Plugin for NormalMaterialPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        embedded_asset!(app, "shaders/normal.wgsl");
        app.add_plugins(MaterialPlugin::<NormalMaterial>::default());
    }
}
