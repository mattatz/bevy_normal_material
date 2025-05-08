use bevy::{
    asset::{weak_handle, Handle},
    prelude::Shader,
};

pub mod material;
pub mod plugin;

pub mod prelude {

    pub use crate::material::NormalMaterial;
    pub use crate::plugin::NormalMaterialPlugin;
}

pub const SHADER_HANDLE: Handle<Shader> = weak_handle!("2608d7fb-50a0-4add-ba74-51bf6ff36c4a");
