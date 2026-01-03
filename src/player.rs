use crate::assets::GameAssets;
use crate::axe::{AXE_Y_HIDDEN, Axe};
use crate::headstone::Headstone;
use crate::input::GameState;
use crate::window::*;
use bevy::audio::{PlaybackMode, Volume};
use bevy::prelude::*;

pub const PLAYER_LEFT_X: f32 = -240.0;
pub const PLAYER_RIGHT_X: f32 = 240.0;
pub const PLAYER_Y: f32 = WINDOW_BOTTOM + 300.0;

pub const PLAYER_Z_INDEX: f32 = 8.0;

pub const PLAYER_Y_HIDDEN: f32 = -2000.0;

#[derive(Component)]
pub struct Player;

#[derive(Copy, Clone, PartialEq, Eq, Debug, PartialOrd)]
pub enum Side {
    Left,
    Right,
    None,
}

#[derive(Component)]
pub struct ObjectSide {
    pub side: Side,
}

#[derive(Event)]
pub struct PlayerDied;

/// Spawns the player at the left side of the tree
pub fn spawn_player(commands: &mut Commands, assets: &Res<GameAssets>) {
    // Spawn the player entity
    commands
        .spawn(Sprite::from_image(assets.player.clone()))
        .insert(Transform::from_xyz(
            PLAYER_RIGHT_X,
            PLAYER_Y,
            PLAYER_Z_INDEX,
        ))
        .insert(ObjectSide { side: Side::Right })
        .insert(Player);
}

pub fn handle_death_sound(_: On<PlayerDied>, mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer::new(assets.death.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(0.4),
            ..Default::default()
        },
    ));
}

pub fn handle_death_player_translation(
    _: On<PlayerDied>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Headstone>)>,
    mut headstone_query: Query<&mut Transform, (With<Headstone>, Without<Player>)>,
) {
    if player_query.iter().len() > 1 {
        panic!("There shouldn't be more than 1 player");
    }

    for mut player_transform in player_query.iter_mut() {
        for mut transform in headstone_query.iter_mut() {
            transform.translation.y = player_transform.translation.y;
            transform.translation.x = player_transform.translation.x;
        }

        player_transform.translation.y = PLAYER_Y_HIDDEN;
    }
}

pub fn handle_game_state_paused(_: On<PlayerDied>, mut next_state: ResMut<NextState<GameState>>) {
    next_state.set(GameState::Paused);
}

pub fn handle_hide_axe_on_death(
    _: On<PlayerDied>,
    mut axe_query: Query<&mut Transform, With<Axe>>,
) {
    for mut axe in axe_query.iter_mut() {
        axe.translation.y = AXE_Y_HIDDEN;
    }
}

pub fn handle_spawn_death_message(
    _: On<PlayerDied>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    spawn_you_died_prompt(&mut commands, &assets);
}
