use bevy::prelude::*;
use bevy_rapier3d::prelude::{ExternalImpulse, Velocity};

#[derive(Component, Default)]
pub struct PidCamera {
  pub pid: Vec3,
  pub integrated_error: Vec3,
  pub last_error: Vec3,
  pub offset: Option<Vec3>,
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
  mut qry_camera: Query<
    (&mut Transform, &mut Velocity, &mut PidCamera),
    (Without<PidCameraTarget>, With<Camera>),
  >,
  time: Res<Time>,
) {
  for (mut cam_transform, mut velocity, mut pid) in qry_camera.iter_mut() {
    if let Ok(target_transform) = qry_transform.get_single() {
      // TODO: interpolate

      let offset = if let Some(o) = pid.offset {
        o
      } else {
        // find camera intersection with Y plane
        let direction = (cam_transform.rotation * Vec3::Z).normalize();
        let t = (Vec3::Y.dot(Vec3::ZERO) - Vec3::Y.dot(cam_transform.translation))
          / Vec3::Y.dot(direction);
        let intersection = cam_transform.translation + (direction * t);

        // calculate offset and move camera translation only
        // since there is no rotation and Y never changes (intersection always has Y = 0 by definition)
        // this hopefully produces a 2d effect
        let o = intersection - cam_transform.translation;
        pid.offset = Some(o);

        info!(
          "new transform {:?}, {:?}, {:?}",
          cam_transform.translation,
          intersection,
          target_transform.translation - o
        );
        info!("direction and magnituted: {:?} {:?}", direction, t);
        o
      };

      let error = target_transform.translation - offset - cam_transform.translation;

      pid.integrated_error += error * time.delta_seconds();
      pid.last_error = error * time.delta_seconds();

      let p = error * pid.pid.x;
      let i =
        (pid.integrated_error * pid.pid.y).clamp(Vec3::new(-100., -100., -100.), Vec3::new(100., 100., 100.));
      let d = ((error - pid.last_error) / time.delta_seconds()) * pid.pid.z;

      velocity.linvel = p + i + d;

      pid.last_error = error;
    }
  }
}
