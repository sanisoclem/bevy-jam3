mod post;
mod toon;
mod cubemap;

use bevy::{
  prelude::*,
};
pub use post::PostProcessSettings;
pub use toon::ToonMaterial;
pub use cubemap::Cubemap;

pub struct VfxPlugin;
impl Plugin for VfxPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(Msaa::Off) // lets see those pixels
      .add_plugin(post::PostProcessPlugin)
      .add_plugin(cubemap::CubemapPlugin)
      .add_plugin(MaterialPlugin::<ToonMaterial>::default());
  }
}
