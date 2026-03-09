use bevy::{
    asset::Asset,
    prelude::{AlphaMode, Material},
    reflect::TypePath,
    render::render_resource::{AsBindGroup, Face, ShaderType},
    shader::ShaderRef,
};

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
    fn vertex_shader() -> ShaderRef {
        ShaderRef::Default
    }

    fn fragment_shader() -> ShaderRef {
        "embedded://bevy_normal_material/shaders/normal.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::mesh::MeshVertexBufferLayoutRef,
        key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.cull_mode = key.bind_group_data.cull_mode;

        #[cfg(feature = "webgl")]
        if let Some(fragment) = &mut descriptor.fragment {
            fragment.shader_defs.push("SIXTEEN_BYTE_ALIGNMENT".into());
        }

        Ok(())
    }
}
