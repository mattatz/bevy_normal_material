use bevy::{
    asset::Asset,
    prelude::{AlphaMode, Material},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, Face, ShaderType},
};

use crate::SHADER_HANDLE;

#[derive(AsBindGroup, Clone, Copy, TypePath, Asset)]
#[bind_group_data(NormalMaterialKey)]
#[uniform(0, NormalMaterialUniform)]
pub struct NormalMaterial {
    pub opacity: f32,
    pub depth_bias: f32,
    pub cull_mode: Option<Face>,
    pub alpha_mode: AlphaMode,
}

#[derive(ShaderType)]
struct NormalMaterialUniform {
    opacity: f32,
    #[cfg(feature = "webgl")]
    _webgl2_padding: bevy::math::Vec3,
}

impl From<&NormalMaterial> for NormalMaterialUniform {
    fn from(material: &NormalMaterial) -> NormalMaterialUniform {
        NormalMaterialUniform {
            opacity: material.opacity,
            #[cfg(feature = "webgl")]
            _webgl2_padding: Default::default(),
        }
    }
}

impl Default for NormalMaterial {
    fn default() -> Self {
        Self {
            opacity: 1.,
            depth_bias: 0.,
            cull_mode: Some(Face::Back),
            alpha_mode: Default::default(),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NormalMaterialKey {
    cull_mode: Option<Face>,
}

impl From<&NormalMaterial> for NormalMaterialKey {
    fn from(material: &NormalMaterial) -> Self {
        NormalMaterialKey {
            cull_mode: material.cull_mode,
        }
    }
}

impl Material for NormalMaterial {
    fn vertex_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Default
    }

    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        bevy::render::render_resource::ShaderRef::Handle(SHADER_HANDLE.clone())
    }

    fn alpha_mode(&self) -> bevy::prelude::AlphaMode {
        self.alpha_mode
    }

    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;

        // WebGL2 structs must be 16 byte aligned.
        let shader_defs = vec![
            #[cfg(feature = "webgl")]
            "SIXTEEN_BYTE_ALIGNMENT".into(),
        ];
        if let Some(fragment) = &mut descriptor.fragment {
            fragment.shader_defs = shader_defs;
        }

        Ok(())
    }
}
