use bevy::prelude::*;

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
  }
}

#[derive(Debug)]
pub enum PlayerCommand {
  Spawn,
  Despawn,
}

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
        cmd.spawn((
          SceneBundle {
            scene: asset_server.load("ship.gltf#Scene0"),
            ..default()
          },
          PlayerComponent,
          PidCameraTarget
        ));
      }
      _ => {
        warn!("Invalid player command {:?}", evt);
      }
    }
  }
}
