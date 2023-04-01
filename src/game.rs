use bevy::prelude::*;
use utils::despawn_screen;

#[derive(Resource)]
struct GameNextState<T>(T);

pub trait GameExtensions {
  fn jam<T: States>(&mut self, game_state: T, exit_state: T) -> &mut Self;
}

impl GameExtensions for App {
  fn jam<T: States>(&mut self, game_state: T, exit_state: T) -> &mut Self {
    self
      .insert_resource(GameNextState(exit_state))
      .add_systems((
        game_setup.in_schedule(OnEnter(game_state.clone())),
        game::<T>.in_set(OnUpdate(game_state.clone())),
        despawn_screen::<OnGameScreen>.in_schedule(OnExit(game_state.clone())),
      ))
  }
}

fn game<T: States>(
  time: Res<Time>,
  next_state: Res<GameNextState<T>>,
  mut game_state: ResMut<NextState<T>>,
  mut timer: ResMut<GameTimer>,
) {
  if timer.tick(time.delta()).finished() {
    game_state.set(next_state.0.clone());
  }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

#[derive(Resource, Deref, DerefMut)]
struct GameTimer(Timer);

fn game_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  commands.insert_resource(GameTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}
