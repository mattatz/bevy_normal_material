use bevy::{
    prelude::{AlphaMode, Material},
    reflect::{TypeUuid, TypePath},
    render::render_resource::{AsBindGroup, Face},
};

use crate::SHADER_HANDLE;

#[derive(AsBindGroup, TypeUuid, Clone, Copy, TypePath)]
#[uuid = "cd561053-324b-4f72-a486-422320cd7ac2"]
#[bind_group_data(NormalMaterialKey)]
pub struct NormalMaterial {
    #[uniform(0)]
    pub opacity: f32,
    pub depth_bias: f32,
    pub cull_mode: Option<Face>,
    pub alpha_mode: AlphaMode,
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
        bevy::render::render_resource::ShaderRef::Handle(SHADER_HANDLE.typed())
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
        _layout: &bevy::render::mesh::MeshVertexBufferLayout,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;

        Ok(())
    }
}
