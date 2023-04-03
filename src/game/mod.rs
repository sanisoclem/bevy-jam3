use bevy::{
  core_pipeline::prepass::{DepthPrepass, NormalPrepass},
  prelude::*,
};
use level::{LevelExtensions, LevelSettings};
use loading::LoadingExtensions;
use utils::vfx::{PostProcessSettings, ToonMaterial};

use self::{camera::PidCamera, player::PlayerExtensions};

mod camera;
mod level;
mod loading;
mod player;

#[derive(Resource)]
struct GameNextState<T>(T);
pub trait GameExtensions {
  fn jam<T: States>(&mut self, game_state: T, exit_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn jam<T: States>(&mut self, game_state: T, _exit_state: T) -> &mut Self {
    self
      .add_state::<GameState>()
      .add_loading_screen(GameState::Loading)
      .add_levels(LevelSettings {
        level_active_state: GameState::Playing,
      })
      .add_player(player::PlayerSettings)
      .add_plugin(camera::PidCameraPlugin)
      .add_systems((
        create_new_game.in_schedule(OnEnter(game_state.clone())),
        // despawn_screen::<OnGameScreen>.in_schedule(OnExit(game_state.clone())),
        // rotate_cam.in_set(OnUpdate(game_state.clone())),
      ))
  }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
  #[default]
  Disabled,
  Playing,
  Loading,
}

fn create_new_game(
  //mut game_time_cmd: EventWriter<GameTimeCommand>,
  mut cmd: Commands,
  mut level_cmd: EventWriter<level::LevelCommand>,
  mut player_cmd: EventWriter<player::PlayerCommand>,
  mut game_state: ResMut<NextState<GameState>>,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ToonMaterial>>,
) {
  // set sate to loading
  game_state.set(GameState::Loading);

  // load the first level
  level_cmd.send(level::LevelCommand::Load(0));

  // spawn the player
  player_cmd.send(player::PlayerCommand::Spawn);

  // spawn the camera
  cmd.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(0.0, 100.0, -2.0).looking_at(Vec3::ZERO, Vec3::Y),
      camera: Camera {
        order: 0,
        ..default()
      },
      ..default()
    },
    DepthPrepass,
    NormalPrepass,
    PostProcessSettings::default(),
    PidCamera { ..default() },
  ));

  // temp so we can see movement
  cmd.spawn(MaterialMeshBundle {
    mesh: meshes.add(shape::Plane::from_size(50.0).into()),
    material: materials.add(ToonMaterial {
      color: Color::rgb(0.3, 0.5, 0.3).into(),
      color_texture: None,
      alpha_mode: AlphaMode::Opaque,
    }),
    transform: Transform::from_xyz(0., -10., 0.),
    ..default()
  });
}
