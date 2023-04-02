use bevy::{
  prelude::*,
  reflect::TypeUuid,
  render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "38e8225d-21ca-4ea3-976d-39b675c5e7a7"]
pub struct ToonMaterial {
  #[uniform(0)]
  pub color: Color,
  #[texture(1)]
  #[sampler(2)]
  pub color_texture: Option<Handle<Image>>,
  pub alpha_mode: AlphaMode,
}

impl Material for ToonMaterial {
  fn fragment_shader() -> ShaderRef {
    "shaders/toon.wgsl".into()
  }

  fn alpha_mode(&self) -> AlphaMode {
    self.alpha_mode
  }

  fn prepass_fragment_shader() -> ShaderRef {
    "shaders/toon_prepass.wgsl".into()
  }
}
