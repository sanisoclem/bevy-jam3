use bevy::ecs::system::WithEntity;
use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_hanabi::prelude::*;
use bevy_hanabi::EffectAsset;
use bevy_mod_raycast::RaycastMesh;
use bevy_rapier3d::prelude::*;

use super::camera::PidCameraTarget; // TODO: make player extensible

pub mod crosshair;

#[derive(Default, Clone, Resource)]
pub struct PlayerSettings;

pub trait PlayerExtensions {
  fn add_player(&mut self, settings: PlayerSettings) -> &mut Self;
}

impl PlayerExtensions for App {
  fn add_player(&mut self, settings: PlayerSettings) -> &mut Self {
    self
      .add_event::<PlayerCommand>()
      .add_event::<PlayerControlCommand>()
      .init_resource::<PlayerState>()
      .insert_resource(settings.clone())
      .add_plugin(crosshair::CrosshairPlugin)
      .add_system(handle_cmd)
      .add_system(read_input)
      .add_system(handle_control_cmd.after(read_input))
      .add_system(show_cotrails.after(handle_control_cmd))
  }
}

#[derive(Debug)]
pub enum PlayerCommand {
  Spawn,
  Despawn,
}

#[derive(Debug)]
pub enum PlayerControlCommand {
  Aim(Vec3),
  Move(Vec3),
  Fire,
  CycleWeapon,
  Shield,
}

#[derive(Component, Default)]
struct PlayerComponent {
  steering_pid: Vec3,
}

#[derive(Resource, Default)]
struct PlayerState {
  current: Option<Entity>,
}

fn handle_cmd(
  mut cmd: Commands,
  mut events: EventReader<PlayerCommand>,
  mut player_state: ResMut<PlayerState>,
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut effects: ResMut<Assets<EffectAsset>>,
) {
  for evt in events.iter() {
    match (evt, player_state.current) {
      (PlayerCommand::Spawn, None) => {
        let crosshair = asset_server.load("crosshair.png");

        let mut color_gradient1 = Gradient::new();
        color_gradient1.add_key(0.0, Vec4::new(0.0, 0.0, 8.0, 1.0));
        color_gradient1.add_key(0.1, Vec4::new(8.0, 8.0, 8.0, 1.0));
        color_gradient1.add_key(0.7, Vec4::new(8.0, 8.0, 0.0, 1.0));
        color_gradient1.add_key(0.9, Vec4::new(8.0, 0.0, 0.0, 1.0));
        color_gradient1.add_key(1.0, Vec4::new(8.0, 0.0, 0.0, 0.0));

        let mut size_gradient1 = Gradient::new();
        size_gradient1.add_key(0.0, Vec2::splat(0.8));
        size_gradient1.add_key(0.3, Vec2::splat(0.5));
        size_gradient1.add_key(1.0, Vec2::splat(0.2));

        // Create the effect asset
        let spawner = Spawner::rate(Value::Uniform((200., 300.)));
        let effect = effects.add(
          EffectAsset {
            name: "cotrails".to_string(),
            // Maximum number of particles alive at a time
            capacity: 32768,
            // Spawn at a rate of 5 particles per second
            spawner,
            ..Default::default()
          }
          // On spawn, randomly initialize the position of the particle
          // to be over the surface of a sphere of radius 2 units.
          .init(InitPositionCone3dModifier {
            base_radius: 0.,
            top_radius: 2.,
            height: 10.,
            dimension: ShapeDimension::Volume,
          })
          .init(InitVelocitySphereModifier {
            center: Vec3::ZERO,
            speed: 50.0.into(),
          })
          // Also initialize the total lifetime of the particle, that is
          // the time for which it's simulated and rendered. This modifier
          // is mandatory, otherwise the particles won't show up.
          .init(InitLifetimeModifier {
            // Give a bit of variation by randomizing the lifetime per particle
            lifetime: Value::Uniform((0.8, 1.2)),
          })
          .init(InitAgeModifier {
            // Give a bit of variation by randomizing the age per particle. This will control the
            // starting color and starting size of particles.
            age: Value::Uniform((0.0, 0.2)),
          })
          .update(LinearDragModifier { drag: 5. })
          .render(BillboardModifier {})
          // Render the particles with a color gradient over their
          // lifetime. This maps the gradient key 0 to the particle spawn
          // time, and the gradient key 1 to the particle death (here, 10s).
          .render(ColorOverLifetimeModifier {
            gradient: color_gradient1,
          })
          .render(SizeOverLifetimeModifier {
            gradient: size_gradient1,
          }),
        );

        let player = cmd
          .spawn((
            SceneBundle {
              scene: asset_server.load("ship.gltf#Scene0"),
              ..default()
            },
            PlayerComponent {
              steering_pid: Vec3::new(1.0, 1.0, 1.0),
            },
            PidCameraTarget,
          ))
          .insert(GravityScale(0.0))
          .insert(RigidBody::Dynamic)
          .insert(Collider::ball(5.0))
          .insert(LockedAxes::TRANSLATION_LOCKED_Y)
          .insert(Damping {
            linear_damping: 5.0,
            angular_damping: 10.0,
          })
          .insert(Dominance::group(10))
          .insert(ColliderMassProperties::Density(1.0))
          .insert(ExternalImpulse {
            impulse: Vec3::new(0.0, 0.0, 0.0),
            torque_impulse: Vec3::new(0.0, 0.0, 0.0),
          })
          .with_children(|b| {
            b.spawn((
              Name::new("emit:cotrails"),
              ParticleEffectBundle {
                effect: ParticleEffect::new(effect),
                transform: Transform::from_translation(Vec3::new(00., 0., 0.0))
                  .with_rotation(Quat::from_rotation_x(-1.)),
                ..Default::default()
              }
              .with_spawner(spawner),
            ));
          })
          .id();

        cmd.spawn((
          ImageBundle {
            style: Style {
              position: UiRect::default(),
              position_type: PositionType::Absolute,
              size: Size::new(Val::Px(50.0), Val::Px(50.0)),
              ..default()
            },
            image: UiImage::new(crosshair),
            visibility: Visibility::Hidden,
            ..default()
          },
          crosshair::Crosshair {
            active: true,
            ..default()
          },
        ));

        cmd.spawn((
          PbrBundle {
            mesh: meshes.add(Mesh::try_from(shape::Plane::from_size(1000000.)).unwrap()),
            material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.0).into()),
            ..Default::default()
          },
          RaycastMesh::<crosshair::CrosshairRaycastSet>::default(),
        ));

        player_state.current = Some(player);
      }
      _ => {
        warn!("Invalid player command {:?}", evt);
      }
    }
  }
}

fn handle_control_cmd(
  mut events: EventReader<PlayerControlCommand>,
  player_state: Res<PlayerState>,
  mut qry: Query<(&Transform, &PlayerComponent, &mut ExternalImpulse)>,
  time: Res<Time>,
) {
  if let Some(player) = player_state.current {
    if let Ok((player_transform, player, mut impulse)) = qry.get_mut(player) {
      for evt in events.iter() {
        match evt {
          PlayerControlCommand::Move(dir) => {
            // todo: create smoothing fn for impulses
            // let max_linear_impulse = 2000.0;
            let max_angular_impulse = 1000.0;
            let multiplier = 2000.0;

            let dir2d = dir.xz().normalize();
            let orientation = (player_transform.rotation * Vec3::Z).normalize();
            let orientation2d = orientation.xz().normalize();
            let error_radians = dir2d.angle_between(orientation2d);
            let error = error_radians.to_degrees();
            let p = error * player.steering_pid.x;
            if error > 135.0 && error < 225.0 {
              //impulse.impulse = orientation * -1.0 * multiplier;
            } else {
              impulse.impulse = orientation * multiplier;
              if error < 135.0 {
                impulse.torque_impulse = Vec3::Y * multiplier * error_radians;
              } else {
                impulse.torque_impulse = Vec3::NEG_Y * multiplier * error_radians;
              }
            }

            //impulse.impulse = *dir * multiplier;
          }
          PlayerControlCommand::Aim(new_pos) => {
            let o = *new_pos - player_transform.translation;
            let t = player_transform.translation - o;
            //player_transform.look_at(t, Vec3::Y);
          }
          _ => {
            warn!("unsupported player cmd {:?}", evt);
          }
        }
      }
    }
  }
}

fn read_input(
  keyboard_input: Res<Input<KeyCode>>,
  mut evts: EventWriter<PlayerControlCommand>,
  qry_crosshair: Query<&crosshair::Crosshair, Changed<crosshair::Crosshair>>,
) {
  let mut move_vec = Vec3::default();

  if keyboard_input.pressed(KeyCode::W) {
    move_vec += Vec3::Z;
  }
  if keyboard_input.pressed(KeyCode::A) {
    move_vec += Vec3::X;
  }
  if keyboard_input.pressed(KeyCode::S) {
    move_vec += Vec3::NEG_Z;
  }
  if keyboard_input.pressed(KeyCode::D) {
    move_vec += Vec3::NEG_X;
  }

  if move_vec.length() > 0.0 {
    evts.send(PlayerControlCommand::Move(move_vec.normalize()));
  }

  for c in qry_crosshair.iter() {
    if let Some(word_pos) = c.world_pos {
      evts.send(PlayerControlCommand::Aim(word_pos));
    }
  }
}

fn show_cotrails(
  qry: Query<(&ExternalImpulse, &Children)>,
  mut qry_effect: Query<&mut ParticleEffect>,
) {
  for (impulse, children) in qry.iter() {
    for child in children {
      if let Ok(mut effect) = qry_effect.get_mut(*child) {
        effect
          .maybe_spawner()
          .unwrap()
          .set_active(impulse.impulse.length() > 0.1);
      }
    }
  }
}
