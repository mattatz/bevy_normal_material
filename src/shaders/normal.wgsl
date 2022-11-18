
struct NormalMaterial {
    opacity: f32,
};

@group(1) @binding(0)
var<uniform> material: NormalMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    var nn = (world_normal + 1.0) * 0.5;
    return vec4(nn, material.opacity);
}