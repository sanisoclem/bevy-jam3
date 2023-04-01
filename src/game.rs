use bevy::prelude::*;
use utils::despawn_screen;

#[derive(Resource)]
struct GameNextState<T>(T);

pub struct GamePlugin<T> {
  game_state: T,
  next_state: T,
}

impl<T> Plugin for GamePlugin<T>
where
  T: Clone + Copy + Eq + PartialEq + States,
{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(GameNextState(self.next_state))
      .add_systems((
        game_setup.in_schedule(OnEnter(self.game_state)),
        Self::game.in_set(OnUpdate(self.game_state)),
        despawn_screen::<OnGameScreen>.in_schedule(OnExit(self.game_state)),
      ));
  }
}

impl<T> GamePlugin<T>
where
  T: Clone + Copy + Eq + PartialEq + States,
{
  pub fn new(game_state: T, next_state: T) -> Self {
    Self {
      game_state,
      next_state,
    }
  }

  fn game(
    time: Res<Time>,
    next_state: Res<GameNextState<T>>,
    mut game_state: ResMut<NextState<T>>,
    mut timer: ResMut<GameTimer>,
  ) {
    if timer.tick(time.delta()).finished() {
      game_state.set(next_state.0);
    }
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
