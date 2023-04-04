use bevy::prelude::*;
use bevy_mod_raycast::RaycastMesh;

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
  mut meshes: ResMut<Assets<Mesh>>,
  asset_server: Res<AssetServer>,
  mut materials: ResMut<Assets<StandardMaterial>>,
) {
  for evt in events.iter() {
    match (evt, player_state.current) {
      (PlayerCommand::Spawn, None) => {
        let crosshair = asset_server.load("crosshair.png");
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
          PlayerControlCommand::FaceLocation(new_pos) => {
            let o = *new_pos - player_transform.translation;
            let t = player_transform.translation - o;
            player_transform.look_at(t, Vec3::Y);
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
  mouse: Res<Input<MouseButton>>,
  qry_crosshair: Query<&crosshair::Crosshair, Changed<crosshair::Crosshair>>,
) {
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

  for c in qry_crosshair.iter() {
    if let Some(word_pos) = c.world_pos {
      evts.send(PlayerControlCommand::FaceLocation(word_pos));
    }
  }
}
