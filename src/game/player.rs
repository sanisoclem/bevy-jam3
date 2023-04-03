use bevy::{math::Vec3Swizzles, prelude::*};

use super::camera::PidCameraTarget; // TODO: make player extensible

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
      .add_system(handle_cmd)
      .add_system(handle_control_cmd)
      .add_system(read_input)
  }
}

#[derive(Debug)]
pub enum PlayerCommand {
  Spawn,
  Despawn,
}

#[derive(Debug)]
pub enum PlayerControlCommand {
  FaceLocation(Vec3),
  Move(Vec3),
  Fire,
  CycleWeapon,
  Shield,
}

#[derive(Component)]
struct PlayerComponent;

#[derive(Resource, Default)]
struct PlayerState {
  current: Option<Entity>,
}

fn handle_cmd(
  mut cmd: Commands,
  mut events: EventReader<PlayerCommand>,
  mut player_state: ResMut<PlayerState>,
  asset_server: Res<AssetServer>,
) {
  for evt in events.iter() {
    match (evt, player_state.current) {
      (PlayerCommand::Spawn, None) => {
        let player = cmd
          .spawn((
            SceneBundle {
              scene: asset_server.load("ship.gltf#Scene0"),
              ..default()
            },
            PlayerComponent,
            PidCameraTarget,
          ))
          .id();
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
  mut qry: Query<&mut Transform, With<PlayerComponent>>,
  time: Res<Time>,
) {
  if let Some(player) = player_state.current {
    if let Ok(mut player_transform) = qry.get_mut(player) {
      for evt in events.iter() {
        match evt {
          PlayerControlCommand::Move(dir) => {
            // todo: create smoothing fn for input
            // todo: don't update translation directly,

            player_transform.translation =
              player_transform.translation + (*dir * 20.) * time.delta_seconds();
          }
          _ => {
            warn!("unsupported player cmd {:?}", evt);
          }
        }
      }
    }
  }
}

fn read_input(keyboard_input: Res<Input<KeyCode>>, mut evts: EventWriter<PlayerControlCommand>) {
  if keyboard_input.pressed(KeyCode::W) {
    evts.send(PlayerControlCommand::Move(Vec3::Z));
  }
  if keyboard_input.pressed(KeyCode::A) {
    evts.send(PlayerControlCommand::Move(Vec3::X));
  }
  if keyboard_input.pressed(KeyCode::S) {
    evts.send(PlayerControlCommand::Move(Vec3::NEG_Z));
  }
  if keyboard_input.pressed(KeyCode::D) {
    evts.send(PlayerControlCommand::Move(Vec3::NEG_X));
  }
}
