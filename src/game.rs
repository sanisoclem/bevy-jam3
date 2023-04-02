use bevy::{
  core_pipeline::prepass::{DepthPrepass, NormalPrepass},
  prelude::*,
};
use utils::despawn_screen;
use utils::vfx::*;

#[derive(Resource)]
struct GameNextState<T>(T);

pub trait GameExtensions {
  fn jam<T: States>(&mut self, game_state: T, exit_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn jam<T: States>(&mut self, game_state: T, _exit_state: T) -> &mut Self {
    self.add_systems((
      setup.in_schedule(OnEnter(game_state.clone())),
      despawn_screen::<OnGameScreen>.in_schedule(OnExit(game_state.clone())),
      rotate_cam.in_set(OnUpdate(game_state.clone())),
    ))
  }
}

#[derive(Component)]
struct OnGameScreen;

fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<ToonMaterial>>,
  _asset_server: Res<AssetServer>,
) {
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
      camera: Camera {
        //target: RenderTarget::Image(image_handle.clone()),
        ..default()
      },
      ..default()
    },
    OnGameScreen,
    DepthPrepass,
    NormalPrepass,
    PostProcessSettings::default(),
    UiCameraConfig { show_ui: false },
  ));
  commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(shape::Plane::from_size(5.0).into()),
    material: materials.add(ToonMaterial {
      color: Color::rgb(0.3, 0.5, 0.3).into(),
      color_texture: None,
      alpha_mode: AlphaMode::Opaque,
    }),
    ..default()
  });
  // cube
  commands.spawn(MaterialMeshBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(ToonMaterial {
      color: Color::rgb(0.8, 0.7, 0.6).into(),
      color_texture: None,
      alpha_mode: AlphaMode::Opaque,
    }),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
  });
  // commands.spawn(SceneBundle {
  //   scene: asset_server.load("ship.gltf#Scene0"),
  //   ..default()
  // });
}

fn rotate_cam(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
  for mut transform in &mut query {
    transform.rotate_around(
      Vec3::ZERO,
      Quat::from_axis_angle(Vec3::Y, 0.55 * time.delta_seconds()),
    );
  }
}
