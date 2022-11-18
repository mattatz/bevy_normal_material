use bevy::{
    prelude::{HandleUntyped, Shader},
    reflect::TypeUuid,
};

pub mod material;
pub mod plugin;

pub mod prelude {

    pub use crate::material::NormalMaterial;
    pub use crate::plugin::NormalMaterialPlugin;
}

pub const SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 17159784698352519001);
