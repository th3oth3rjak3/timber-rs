use std::time::Duration;

use bevy::prelude::*;

use crate::{
    chop_system::Chop,
    events::{OutOfTime, StartGame},
    input::GameState,
};

#[derive(Resource)]
pub struct GameTimer {
    pub timer: Timer,
    pub max_seconds: f32,
}

impl GameTimer {
    pub fn new(seconds: f32) -> Self {
        Self {
            timer: Timer::from_seconds(seconds, TimerMode::Once),
            max_seconds: seconds,
        }
    }
}

#[derive(Component)]
pub struct TimerBar;

pub fn spawn_timer_bar(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(30.0),
            left: Val::Percent(50.0),
            width: Val::Px(400.0),
            height: Val::Px(25.0),
            margin: UiRect {
                left: Val::Px(-200.0), // half width → center
                ..default()
            },
            ..default()
        },
        BackgroundColor(Color::srgb(0.8, 0.0, 0.0)),
        TimerBar,
    ));
}

pub fn tick_game_timer(
    time: Res<Time>,
    mut commands: Commands,
    mut game_timer: ResMut<GameTimer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    game_timer.timer.tick(time.delta());
    if game_timer.timer.remaining_secs() == 0.0 {
        next_state.set(GameState::Paused);
        commands.trigger(OutOfTime);
    }
}

pub fn update_timer_bar(game_timer: Res<GameTimer>, mut query: Query<&mut Node, With<TimerBar>>) {
    let remaining = game_timer.timer.remaining_secs();
    let ratio = (remaining / game_timer.max_seconds).clamp(0.0, 1.0);

    for mut node in query.iter_mut() {
        node.width = Val::Px(400.0 * ratio);
        node.margin.left = Val::Px(-200.0 * ratio); // keep centered
    }
}

pub fn add_time_on_chop(_: On<Chop>, mut game_timer: ResMut<GameTimer>) {
    let bonus = 0.25; // quarter second per chop

    let new_remaining = (game_timer.timer.remaining_secs() + bonus).min(game_timer.max_seconds);

    let max_seconds = game_timer.max_seconds;
    game_timer
        .timer
        .set_elapsed(Duration::from_secs_f32(max_seconds - new_remaining));
}

pub fn reset_game_timer(_: On<StartGame>, mut timer: ResMut<GameTimer>) {
    timer.timer.reset();
}
