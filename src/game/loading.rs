use bevy::prelude::*;
use utils::despawn_screen;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

pub trait LoadingExtensions {
  fn add_loading_screen<T: States>(&mut self, show_on_state: T) -> &mut Self;
}

impl LoadingExtensions for App {
  fn add_loading_screen<T: States>(&mut self, show_on_state: T) -> &mut Self {
    self
      .add_system(setup.in_schedule(OnEnter(show_on_state.clone())))
      .add_system(despawn_screen::<OnLoadingScreen>.in_schedule(OnExit(show_on_state.clone())))
  }
}

#[derive(Component)]
struct OnLoadingScreen;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  // let font = asset_server.load("fonts/FiraSans-Bold.ttf");

  // commands
  //   .spawn((
  //     NodeBundle {
  //       style: Style {
  //         align_items: AlignItems::Center,
  //         justify_content: JustifyContent::Center,
  //         size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
  //         ..default()
  //       },
  //       ..default()
  //     },
  //     OnLoadingScreen,
  //   )).with_children(|b| {
  //     b.spawn(
  //       TextBundle::from_section(
  //         "Loading...",
  //         TextStyle {
  //           font: font.clone(),
  //           font_size: 80.0,
  //           color: TEXT_COLOR,
  //         },
  //       )
  //       .with_style(Style {
  //         margin: UiRect::all(Val::Px(50.0)),
  //         ..default()
  //       }),
  //     );
  //   });
}

