use bevy::prelude::*;

#[derive(Component, Default)]
pub struct PidCamera {
  pub pid: Vec3,
}

#[derive(Component)]
pub struct PidCameraTarget;

pub struct PidCameraPlugin;
impl Plugin for PidCameraPlugin {
  fn build(&self, app: &mut App) {
    app.add_system(follow_target);
  }
}

fn follow_target(
  qry_transform: Query<&Transform, (With<PidCameraTarget>, Without<Camera>)>,
  mut qry_camera: Query<(&mut Transform, &PidCamera), (Without<PidCameraTarget>, With<Camera>)>,
) {
  for (mut cam_transform, pid) in qry_camera.iter_mut() {
    if let Ok(target_transform) = qry_transform.get_single() {
      // TODO: interpolate

      // find camera intersection with Y plane
      let direction = (cam_transform.rotation * Vec3::Z).normalize();
      let t = (Vec3::Y.dot(Vec3::ZERO) - Vec3::Y.dot(cam_transform.translation))
        / Vec3::Y.dot(direction);
      let intersection = cam_transform.translation + (direction * t);

      // calculate offset and move camera translation only
      // since there is no rotation and Y never changes (intersection always has Y = 0 by definition)
      // this hopefully produces a 2d effect
      let offset = intersection - cam_transform.translation;
      //cam_transform.translation = target_transform.translation + offset;
      // info!(
      //   "new transform {:?}, {:?}, {:?}",
      //   cam_transform.translation,
      //   intersection,
      //   target_transform.translation + offset
      // );
      // info!("direction and magnituted: {:?} {:?}", direction, t);
    }
  }
}
