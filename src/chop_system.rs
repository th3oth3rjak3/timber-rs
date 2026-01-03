use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::{
    assets::GameAssets,
    axe::{AXE_LEFT_X, AXE_RIGHT_X, AXE_Y, Axe},
    branch::{
        BRANCH_LEFT_X, BRANCH_RIGHT_X, Branch, Index, get_branch_y_position, get_random_branch_side,
    },
    log::spawn_log,
    player::{ObjectSide, PLAYER_LEFT_X, PLAYER_RIGHT_X, PLAYER_Y, Player, PlayerDied, Side},
    score::Score,
};

#[derive(Event)]
pub struct Chop {
    pub side: Side,
}

pub fn handle_player_branch_interaction(
    chop: On<Chop>,
    mut commands: Commands,
    mut player_query: Query<(&mut Transform, &mut ObjectSide), (With<Player>, Without<Branch>)>,
    mut branch_query: Query<
        (&mut Transform, &mut Index, &mut ObjectSide),
        (With<Branch>, Without<Player>),
    >,
) {
    let (mut player_transform, mut player_side) = player_query.single_mut().unwrap();
    match chop.side {
        Side::Left => {
            player_transform.translation.x = PLAYER_LEFT_X;
            player_transform.translation.y = PLAYER_Y;
            if player_side.side == Side::Right {
                player_transform.scale.x = -1.0;
            }
            player_side.side = Side::Left;
        }
        Side::Right => {
            player_transform.translation.x = PLAYER_RIGHT_X;
            player_transform.translation.y = PLAYER_Y;
            if player_side.side == Side::Left {
                player_transform.scale.x = 1.0;
            }
            player_side.side = Side::Right;
        }
        Side::None => {}
    };

    for (mut transform, mut index, mut side) in branch_query.iter_mut() {
        if index.0 == 5 {
            index.0 = 0;
            let new_side = get_random_branch_side();

            match new_side {
                Side::Left => transform.scale.x = -1.0,
                Side::Right | Side::None => transform.scale.x = 1.0,
            };

            side.side = new_side;
            match new_side {
                Side::Left => {
                    transform.translation.x = BRANCH_LEFT_X;
                    transform.translation.y = get_branch_y_position(index.0);
                }
                Side::Right => {
                    transform.translation.x = BRANCH_RIGHT_X;
                    transform.translation.y = get_branch_y_position(index.0);
                }
                Side::None => {
                    transform.translation.x = -2000.0;
                    transform.translation.y = -2000.0;
                }
            };
        } else {
            index.0 += 1;
            // just move the branch down the tree
            transform.translation.y = get_branch_y_position(index.0);
        }

        // Player Died
        if index.0 == 5 && side.side == player_side.side {
            commands.trigger(PlayerDied);
        }
    }
}

pub fn play_chop_sound(_: On<Chop>, mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer::new(assets.chop.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(0.7),
            ..Default::default()
        },
    ));
}

pub fn increase_player_score(_: On<Chop>, mut score_query: Query<&mut Score>) {
    for mut score in score_query.iter_mut() {
        score.0 += 1;
    }
}

pub fn show_axe_sprite(chop: On<Chop>, mut axe_query: Query<&mut Transform, With<Axe>>) {
    let mut transform = axe_query.single_mut().unwrap();
    match chop.side {
        Side::Left => {
            transform.scale.x = -1.0;
            transform.translation.y = AXE_Y;
            transform.translation.x = AXE_LEFT_X;
        }
        Side::Right => {
            transform.scale.x = 1.0;
            transform.translation.y = AXE_Y;
            transform.translation.x = AXE_RIGHT_X;
        }
        Side::None => {}
    };
}

pub fn handle_log_active(chop: On<Chop>, mut commands: Commands, assets: Res<GameAssets>) {
    let log_side = match chop.side {
        Side::Left => Side::Right,
        Side::Right => Side::Left,
        Side::None => Side::None,
    };
    spawn_log(&mut commands, &assets, 5.0, log_side);
}
