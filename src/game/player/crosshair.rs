use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};
use bevy_mod_raycast::{DefaultRaycastingPlugin, RaycastMethod, RaycastSource, RaycastSystem};

pub struct CrosshairPlugin;
impl Plugin for CrosshairPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugin(DefaultRaycastingPlugin::<CrosshairRaycastSet>::default())
      .add_system(
        update_crosshair_screen_pos
          .in_base_set(CoreSet::First)
          .before(RaycastSystem::BuildRays::<CrosshairRaycastSet>),
      )
      .add_system(read_input)
      .add_system(update_crosshair_visibility.after(read_input))
      .add_system(update_crosshair_world_pos);
  }
}

#[derive(Clone, Reflect)]
pub struct CrosshairRaycastSet;

#[derive(Component, Default)]
pub struct Crosshair {
  pub active: bool,
  pub last_pos: Option<Vec2>,
  pub world_pos: Option<Vec3>,
}

fn read_input(
  mut qry_crosshair: Query<&mut Crosshair>,
  keyboard_input: Res<Input<KeyCode>>,
  mouse: Res<Input<MouseButton>>,
) {
  if let Ok(mut c) = qry_crosshair.get_single_mut() {
    if mouse.just_pressed(MouseButton::Left) {
      c.active = true;
    }

    if keyboard_input.just_pressed(KeyCode::Escape) {
      c.active = false;
    }
  }
}

fn update_crosshair_visibility(
  mut qry_crosshair: Query<(&mut Crosshair, &mut Style, &mut Visibility), Changed<Crosshair>>,
  mut windows: Query<&mut Window>,
) {
  let mut window = windows.single_mut();
  // refactor to use cmds if crosshair interaction gets complicated

  if let Ok((mut c, mut style, mut v)) = qry_crosshair.get_single_mut() {
    if c.active {
      window.cursor.visible = false;
      window.cursor.grab_mode = CursorGrabMode::Locked;
      *v = Visibility::Visible;

      if c.last_pos == None {
        let cursor_pos = if let Some(cp) = window.cursor_position() {
          cp
        } else {
          Vec2::new(
            window.resolution.width() / 2.0,
            window.resolution.height() / 2.0,
          )
        };
        c.last_pos = Some(cursor_pos);

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

fn update_crosshair_screen_pos(
  mut mouse_motion_events: EventReader<MouseMotion>,
  mut qry_crosshair: Query<(&mut Crosshair, &mut Style)>,
  mut qry_raycast: Query<&mut RaycastSource<CrosshairRaycastSet>>,
  windows: Query<&Window>,
) {
  let window = windows.single();

  for event in mouse_motion_events.iter() {
    if let Ok((mut c, mut style)) = qry_crosshair.get_single_mut() {
      if !c.active {
        break;
      }

      // TODO: send player commend to orient ship
      match (c.last_pos, style.position.left, style.position.bottom) {
        (Some(last_pos), Val::Px(l), Val::Px(b)) => {
          let new_pos = Vec2::new(last_pos.x + event.delta.x, last_pos.y - event.delta.y).clamp(
            Vec2::default(),
            Vec2::new(window.resolution.width(), window.resolution.height()),
          );
          c.last_pos = Some(new_pos);
          style.position = UiRect::new(
            Val::Px(l + event.delta.x),
            Val::Undefined,
            Val::Undefined,
            Val::Px(b - event.delta.y),
          );

          for mut pick_source in &mut qry_raycast {
            pick_source.cast_method = RaycastMethod::Screenspace(new_pos);
          }
        }
        _ => {
          warn!("cannot update crosshair pos");
        }
      }
    }
  }
}

fn update_crosshair_world_pos(
  mut crosshair: Query<&mut Crosshair>,
  to: Query<&RaycastSource<CrosshairRaycastSet>>,
) {
  if let Ok(raycast_source) = to.get_single() {
    if let Some(top_intersection) = raycast_source.get_nearest_intersection() {
      let mut new_pos = top_intersection.1.position();
      new_pos.y = 0.0;

      if let Ok(mut c) = crosshair.get_single_mut() {
        c.world_pos = Some(new_pos)
      }
    }
  }
}
