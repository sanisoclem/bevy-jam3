use bevy::prelude::*;

pub mod vfx;
pub mod game_time;
pub mod ship;
// pub mod grid;

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
  for entity in &to_despawn {
    commands.entity(entity).despawn_recursive();
  }
}