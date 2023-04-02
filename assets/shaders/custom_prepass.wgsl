#import bevy_pbr::mesh_view_types
#import bevy_pbr::mesh_types

@group(0) @binding(0)
var<uniform> view: View;

struct ToonMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: ToonMaterial;
@group(1) @binding(1)
var base_color_texture: texture_2d<f32>;
@group(1) @binding(2)
var base_color_sampler: sampler;

@group(2) @binding(0)
var<uniform> mesh: Mesh;

#ifdef SKINNED
@group(2) @binding(1)
var<uniform> joint_matrices: SkinnedMesh;
#import bevy_pbr::skinning
#endif

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
#ifdef VERTEX_UVS
    @location(0) uv: vec2<f32>,
#endif // VERTEX_UVS
#ifdef NORMAL_PREPASS
    @location(1) world_normal: vec3<f32>,
#ifdef VERTEX_TANGENTS
    @location(2) world_tangent: vec4<f32>,
#endif // VERTEX_TANGENTS
#endif // NORMAL_PREPASS
};

// Cutoff used for the premultiplied alpha modes BLEND and ADD.
const PREMULTIPLIED_ALPHA_CUTOFF = 0.05;


#ifdef NORMAL_PREPASS
@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {  
  return vec4(in.world_normal * 0.5 + vec3(0.5), 1.0);
}

#else // NORMAL_PREPASS

@fragment
fn fragment(in: FragmentInput) {
}

#endif // NORMAL_PREPASS

