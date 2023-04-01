use bevy::prelude::*;
use game::GameExtensions;
use menu::MenuExtensions;
use splash::SplashExtensions;

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
    .add_splash_screen(GameState::Splash, GameState::Menu)
    .add_main_menu(GameState::Menu, GameState::Game)
    .jam(GameState::Game, GameState::Menu)
    .run();
}

fn setup(mut commands: Commands) {
  commands.spawn(Camera2dBundle::default());
}
