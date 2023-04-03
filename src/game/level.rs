use bevy::prelude::*;
use utils::despawn_screen;

#[derive(Resource, Clone)]
pub struct LevelSettings<T> {
  pub level_active_state: T,
}

pub trait LevelExtensions {
  fn add_levels<T: States>(&mut self, settings: LevelSettings<T>) -> &mut Self;
}

impl LevelExtensions for App {
  fn add_levels<T: States>(&mut self, settings: LevelSettings<T>) -> &mut Self {
    self
      .add_event::<LevelCommand>()
      .init_resource::<LevelState>()
      .insert_resource(settings.clone())
      .add_system(handle_cmd)
      .add_system(despawn_screen::<OnLevel>.in_schedule(OnExit(settings.level_active_state.clone())))
  }
}

#[derive(Component)]
struct OnLevel;

#[derive(Debug)]
pub enum LevelCommand {
  Load(u64),
  Show,
  Unload,
}

#[derive(Resource, Default, Copy, Clone, Debug)]
pub enum LevelState {
  #[default]
  Unloaded, // nothing is loaded
  Loading(u64), // loading a level
  Loaded(u64),  // level is loaded but not shown
  Active(u64),  // level is loaded and shown
}

fn handle_cmd(mut events: EventReader<LevelCommand>, mut level_state: ResMut<LevelState>) {
  for evt in events.iter() {
    match (evt, *level_state) {
      (LevelCommand::Load(level_id), _) => {
        // TODO: load level
        *level_state = LevelState::Loading(*level_id);

        // after level is loaded, transition to settings.level_active_state and set state to loaded
      }
      (LevelCommand::Show, LevelState::Loaded(level_id)) => {
        // only set to active if already loaded
        *level_state = LevelState::Active(level_id)
      }
      (LevelCommand::Unload, _) => *level_state = LevelState::Unloaded,
      _ => {
        warn!("invalid level command, {:?}", evt);
      }
    }
  }
}

// TODO: create despawn system