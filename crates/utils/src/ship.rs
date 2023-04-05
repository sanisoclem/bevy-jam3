use bevy::prelude::*;

#[derive(Component)]
pub struct Spaceship {
  velocity: Vec3,
  mass: f32,
  net_forces: Vec3,
}



// pub fn move_ships(mut qry: Query<(&mut Transform, &mut Spaceship)>) {
//   for (mut transform, mut ship) in qry.iter_mut() {
//     let net_force = if ship.forces.length() < 0.0001 { ship.dampener_strength }
//     let acc = ship.forces
//   }
// }