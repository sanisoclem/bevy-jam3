use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
  #[default]
  Splash,
  Menu,
  Game,
}

mod game;
mod menu;
mod splash;

fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_startup_system(setup)
    .add_state::<GameState>()
    .add_plugin(splash::SplashPlugin::new(
      GameState::Splash,
      GameState::Menu,
    ))
    .add_plugin(menu::MenuPlugin::new(GameState::Menu, GameState::Game))
    .add_plugin(game::GamePlugin::new(GameState::Game, GameState::Menu))
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
