use bevy::prelude::*;
use utils::despawn_screen;

pub trait SplashExtensions {
  fn add_splash_screen<T: States>(&mut self, show_on_state: T, next_state: T) -> &mut Self;
}

impl SplashExtensions for App {
  fn add_splash_screen<T: States>(&mut self, show_on_state: T, next_state: T) -> &mut Self {
    self
      .insert_resource(SplashNextState(next_state))
      .add_system(splash_setup.in_schedule(OnEnter(show_on_state.clone())))
      .add_system(countdown::<T>.in_set(OnUpdate(show_on_state.clone())))
      .add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(show_on_state.clone())))
  }
}

#[derive(Component)]
struct OnSplashScreen;

#[derive(Resource, Deref, DerefMut)]
struct SplashTimer(Timer);

#[derive(Resource)]
struct SplashNextState<T>(T);

fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
  let icon = asset_server.load("splash.png");

  commands
    .spawn((
      NodeBundle {
        style: Style {
          align_items: AlignItems::Center,
          justify_content: JustifyContent::Center,
          size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
          ..default()
        },
        ..default()
      },
      OnSplashScreen,
    ))
    .with_children(|parent| {
      parent.spawn(ImageBundle {
        style: Style {
          size: Size::new(Val::Px(200.0), Val::Auto),
          ..default()
        },
        image: UiImage::new(icon),
        ..default()
      });
    });

  commands.insert_resource(SplashTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn countdown<T: States>(
  mut timer: ResMut<SplashTimer>,
  mut game_state: ResMut<NextState<T>>,
  next_state: Res<SplashNextState<T>>,
  time: Res<Time>,
) {
  if timer.tick(time.delta()).finished() {
    game_state.set(next_state.0.clone());
  }
}
