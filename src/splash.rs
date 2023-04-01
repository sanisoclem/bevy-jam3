use bevy::prelude::*;
use utils::despawn_screen;

pub struct SplashPlugin<T> {
  splash_state: T,
  next_state: T,
}

impl<T> Plugin for SplashPlugin<T>
where
  T: Clone + Copy + Eq + PartialEq + States,
{
  fn build(&self, app: &mut App) {
    app
      .insert_resource(SplashNextState(self.next_state))
      .add_system(splash_setup.in_schedule(OnEnter(self.splash_state)))
      .add_system(Self::countdown.in_set(OnUpdate(self.splash_state)))
      .add_system(despawn_screen::<OnSplashScreen>.in_schedule(OnExit(self.splash_state)));
  }
}
impl<T> SplashPlugin<T>
where
  T: Clone + Copy + Eq + PartialEq + States,
{
  pub fn new(splash_state: T, next_state: T) -> Self {
    Self {
      splash_state,
      next_state,
    }
  }

  fn countdown(
    mut timer: ResMut<SplashTimer>,
    mut game_state: ResMut<NextState<T>>,
    next_state: Res<SplashNextState<T>>,
    time: Res<Time>,
  ) {
    if timer.tick(time.delta()).finished() {
      game_state.set(next_state.0);
    }
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
