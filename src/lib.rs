use bevy::{asset::Handle, prelude::Shader};

pub mod material;
pub mod plugin;

pub mod prelude {

    pub use crate::material::NormalMaterial;
    pub use crate::plugin::NormalMaterialPlugin;
}

pub const SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(17159784698352519001);
