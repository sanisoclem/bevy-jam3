use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

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
      .add_system(read_input)
      .add_system(handle_control_cmd.after(read_input))
      .add_system(update_crosshair.after(read_input))
      .add_system(move_crosshair.after(update_crosshair))
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

#[derive(Component, Default)]
struct Crosshair {
  active: bool,
}

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
          Crosshair { active: true },
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
  mut qry_crosshair: Query<&mut Crosshair>,
  mouse: Res<Input<MouseButton>>,
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

  if let Ok(mut c) = qry_crosshair.get_single_mut() {
    if mouse.just_pressed(MouseButton::Left) {
      c.active = true;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
      c.active = false;
    }
  }
}

fn update_crosshair(
  mut qry_crosshair: Query<(&Crosshair, &mut Style, &mut Visibility), Changed<Crosshair>>,
  mut windows: Query<&mut Window>,
) {
  let mut window = windows.single_mut();

  if let Ok((c, mut style, mut v)) = qry_crosshair.get_single_mut() {
    if c.active {
      window.cursor.visible = false;
      window.cursor.grab_mode = CursorGrabMode::Locked;
      *v = Visibility::Visible;
      
      if style.position.left == Val::Undefined || style.position.bottom == Val::Undefined {
        let cursor_pos = if let Some(cp) = window.cursor_position() {
          cp
        } else {
          Vec2::new(
            window.resolution.width() / 2.0,
            window.resolution.height() / 2.0,
          )
        };

        let w = if let Val::Px(r) = style.size.width {
          r / 2.0
        } else {
          0.0
        };
        let h = if let Val::Px(r) = style.size.height {
          r / 2.0
        } else {
          0.0
        };

        style.position = UiRect::new(
          Val::Px(cursor_pos.x - w),
          Val::Undefined,
          Val::Undefined,
          Val::Px(cursor_pos.y - h),
        );
      }
    } else {
      window.cursor.visible = true;
      window.cursor.grab_mode = CursorGrabMode::None;
      *v = Visibility::Hidden;
    }
  }
}

fn move_crosshair(
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut qry_crosshair: Query<(&Crosshair, &mut Style)>,
) {
  for event in mouse_motion_events.iter() {
    if let Ok((c, mut style)) = qry_crosshair.get_single_mut() {
      if !c.active {
        break;
      }

      // TODO: send player commend to orient ship
      match (style.position.left, style.position.bottom) {
        (Val::Px(l), Val::Px(b)) => {
          style.position = UiRect::new(
            Val::Px(l + event.delta.x),
            Val::Undefined,
            Val::Undefined,
            Val::Px(b - event.delta.y),
          );
        }
        _ => {
          warn!("cannot update crosshair pos");
        }
      }
    }
  }
}
