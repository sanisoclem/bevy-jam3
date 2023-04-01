use bevy::{app::AppExit, prelude::*};
use utils::despawn_screen;

const TEXT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
  Main,
  #[default]
  Disabled,
}

#[derive(Resource)]
struct MenuNextState<T>(T);

pub trait MenuExtensions {
  fn add_main_menu<T: States>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl MenuExtensions for App {
  fn add_main_menu<T: States>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .add_state::<MenuState>()
      .insert_resource(MenuNextState(next_state))
      .add_system(menu_setup.in_schedule(OnEnter(show_on_state.clone())))
      .add_systems((
        main_menu_setup.in_schedule(OnEnter(MenuState::Main)),
        despawn_screen::<OnMainMenuScreen>.in_schedule(OnExit(MenuState::Main)),
      ))
      .add_systems((menu_action::<T>, button_system).in_set(OnUpdate(show_on_state.clone())))
  }
}

fn menu_action<T: States>(
  interaction_query: Query<(&Interaction, &MenuButtonAction), (Changed<Interaction>, With<Button>)>,
  mut app_exit_events: EventWriter<AppExit>,
  mut menu_state: ResMut<NextState<MenuState>>,
  mut game_state: ResMut<NextState<T>>,
  next_state: Res<MenuNextState<T>>,
) {
  for (interaction, menu_button_action) in &interaction_query {
    if *interaction == Interaction::Clicked {
      match menu_button_action {
        MenuButtonAction::Quit => app_exit_events.send(AppExit),
        MenuButtonAction::Play => {
          game_state.set(next_state.0.clone());
          menu_state.set(MenuState::Disabled);
        }
      }
    }
  }
}

#[derive(Component)]
struct OnMainMenuScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct SelectedOption;

#[derive(Component)]
enum MenuButtonAction {
  Play,
  Quit,
}

fn button_system(
  mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
    (Changed<Interaction>, With<Button>),
  >,
) {
  for (interaction, mut color, selected) in &mut interaction_query {
    *color = match (*interaction, selected) {
      (Interaction::Clicked, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
      (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
      (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
      (Interaction::None, None) => NORMAL_BUTTON.into(),
    }
  }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
  menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let font = asset_server.load("fonts/FiraSans-Bold.ttf");
  // Common style for all buttons on the screen
  let button_style = Style {
    size: Size::new(Val::Px(250.0), Val::Px(65.0)),
    margin: UiRect::all(Val::Px(20.0)),
    justify_content: JustifyContent::Center,
    align_items: AlignItems::Center,
    ..default()
  };
  let button_icon_style = Style {
    size: Size::new(Val::Px(30.0), Val::Auto),
    // This takes the icons out of the flexbox flow, to be positioned exactly
    position_type: PositionType::Absolute,
    // The icon will be close to the left border of the button
    position: UiRect {
      left: Val::Px(10.0),
      right: Val::Auto,
      top: Val::Auto,
      bottom: Val::Auto,
    },
    ..default()
  };
  let button_text_style = TextStyle {
    font: font.clone(),
    font_size: 40.0,
    color: TEXT_COLOR,
  };

  commands
    .spawn((
      NodeBundle {
        style: Style {
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          ..default()
        },
        ..default()
      },
      OnMainMenuScreen,
    ))
    .with_children(|parent| {
      parent
        .spawn(NodeBundle {
          style: Style {
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            ..default()
          },
          background_color: Color::CRIMSON.into(),
          ..default()
        })
        .with_children(|parent| {
          // Display the game name
          parent.spawn(
            TextBundle::from_section(
              "Bevy Game Jam #3",
              TextStyle {
                font: font.clone(),
                font_size: 80.0,
                color: TEXT_COLOR,
              },
            )
            .with_style(Style {
              margin: UiRect::all(Val::Px(50.0)),
              ..default()
            }),
          );

          parent
            .spawn((
              ButtonBundle {
                style: button_style.clone(),
                background_color: NORMAL_BUTTON.into(),
                ..default()
              },
              MenuButtonAction::Play,
            ))
            .with_children(|parent| {
              let icon = asset_server.load("ui/right.png");
              parent.spawn(ImageBundle {
                style: button_icon_style.clone(),
                image: UiImage::new(icon),
                ..default()
              });
              parent.spawn(TextBundle::from_section("Play", button_text_style.clone()));
            });

          parent
            .spawn((
              ButtonBundle {
                style: button_style,
                background_color: NORMAL_BUTTON.into(),
                ..default()
              },
              MenuButtonAction::Quit,
            ))
            .with_children(|parent| {
              let icon = asset_server.load("ui/exitRight.png");
              parent.spawn(ImageBundle {
                style: button_icon_style,
                image: UiImage::new(icon),
                ..default()
              });
              parent.spawn(TextBundle::from_section("Quit", button_text_style));
            });
        });
    });
}
