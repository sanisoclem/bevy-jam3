use bevy::prelude::*;
use game::GameExtensions;
use menu::MenuExtensions;
use splash::SplashExtensions;
use utils::{vfx::VfxPlugin, game_time::GameTimePlugin};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
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
    .add_state::<AppState>()
    .add_plugin(VfxPlugin)
    .add_plugin(GameTimePlugin)
    .add_splash_screen(AppState::Splash, AppState::Menu)
    .add_main_menu(AppState::Menu, AppState::Game)
    .jam(AppState::Game, AppState::Menu)
    .run();
}
