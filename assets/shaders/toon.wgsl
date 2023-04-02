#import bevy_pbr::mesh_types
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::prepass_utils

struct ToonMaterial {
  color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ToonMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@fragment
fn fragment(
  @builtin(position) frag_coord: vec4<f32>,
  @builtin(sample_index) sample_index: u32,
  #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
  var n: array<vec3<f32>,9>;
  return material.color * textureSample(base_color_texture, base_color_sampler, uv);
}
