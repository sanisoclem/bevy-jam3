use bevy::prelude::*;
use game::GameExtensions;
use menu::MenuExtensions;
use splash::SplashExtensions;
use utils::vfx::VfxPlugin;

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
    .add_plugins(DefaultPlugins.set(AssetPlugin {
      watch_for_changes: true,
      ..default()
    }))
    .add_state::<GameState>()
    .add_plugin(VfxPlugin)
    .add_splash_screen(GameState::Splash, GameState::Menu)
    .add_main_menu(GameState::Menu, GameState::Game)
    .jam(GameState::Game, GameState::Menu)
    .run();
}
