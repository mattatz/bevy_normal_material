#import bevy_pbr::forward_io::{VertexOutput}

struct NormalMaterial {
    opacity: f32,
#ifdef SIXTEEN_BYTE_ALIGNMENT
    // WebGL2 structs must be 16 byte aligned.
    _webgl2_padding: vec3<f32>
#endif
};

@group(2) @binding(0)
var<uniform> material: NormalMaterial;

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    var nn = (mesh.world_normal + 1.0) * 0.5;
    return vec4(nn, material.opacity);
}