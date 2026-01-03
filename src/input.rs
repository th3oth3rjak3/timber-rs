use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    axe::{AXE_Y_HIDDEN, Axe},
    branch::{Branch, spawn_branches},
    chop_system::Chop,
    events::StartGame,
    headstone::{HEADSTONE_Y_HIDDEN, Headstone},
    player::{PLAYER_Y, Player, Side},
};

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum GameState {
    Playing,
    #[default]
    Paused,
}

pub fn handle_player_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut commands: Commands,
    mut exit_writer: MessageWriter<AppExit>,
    mut axe_query: Query<&mut Transform, With<Axe>>,
) {
    if keys.just_pressed(KeyCode::ArrowLeft) && *state.get() == GameState::Playing {
        commands.trigger(Chop { side: Side::Left });
    }

    if keys.just_pressed(KeyCode::ArrowRight) && *state.get() == GameState::Playing {
        commands.trigger(Chop { side: Side::Right });
    }

    if keys.just_pressed(KeyCode::Enter) && *state.get() == GameState::Paused {
        commands.trigger(StartGame);
    }

    if keys.just_pressed(KeyCode::Escape) {
        exit_writer.write(AppExit::Success);
    }

    if keys.just_released(KeyCode::ArrowLeft) && *state.get() == GameState::Playing {
        for mut transform in axe_query.iter_mut() {
            transform.translation.y = AXE_Y_HIDDEN;
        }
    }

    if keys.just_released(KeyCode::ArrowRight) && *state.get() == GameState::Playing {
        for mut transform in axe_query.iter_mut() {
            transform.translation.y = AXE_Y_HIDDEN;
        }
    }
}

pub fn handle_start_game(_: On<StartGame>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Playing);
}

pub fn handle_reset_player_sprite(
    _: On<StartGame>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_query.single_mut().unwrap();
    transform.translation.y = PLAYER_Y;
}

pub fn handle_reset_headstone_sprite(
    _: On<StartGame>,
    mut headstone_query: Query<&mut Transform, With<Headstone>>,
) {
    let mut transform = headstone_query.single_mut().unwrap();
    transform.translation.y = HEADSTONE_Y_HIDDEN;
}

pub fn handle_reset_branches(
    _: On<StartGame>,
    mut commands: Commands,
    assets: Res<GameAssets>,
    mut branch_query: Query<Entity, With<Branch>>,
) {
    for entity in branch_query.iter_mut() {
        commands.entity(entity).despawn();
    }

    spawn_branches(&mut commands, &assets);
}
