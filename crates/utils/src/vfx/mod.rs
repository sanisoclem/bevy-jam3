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
      .add_plugin(MaterialPlugin::<ToonMaterial>::default())
      .add_system(mod_scene);
  }
}


#[derive(Component)]
struct Inserted;

fn mod_scene(
  mut commands: Commands,
  qry: Query<(Entity, &Handle<StandardMaterial>, &Name), Without<Inserted>>,
  mut toon_materials: ResMut<Assets<ToonMaterial>>,
  std_materials: Res<Assets<StandardMaterial>>,
) {
  for (entity, mat_handle, name) in qry.iter() {
    let old_mat = std_materials.get(mat_handle).unwrap();
    info!("patching {}", name);

    let mat = toon_materials.add(ToonMaterial {
      color: old_mat.base_color,
      color_texture: old_mat.base_color_texture.clone(),
      alpha_mode: AlphaMode::Opaque,
    });
    commands
      .entity(entity)
      .remove::<Handle<StandardMaterial>>()
      .insert(mat)
      .insert(Inserted);
  }
}
