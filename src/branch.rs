use bevy::prelude::*;
use rand::random_range;

use crate::{
    assets::GameAssets,
    player::{ObjectSide, Side},
    window::{WINDOW_LEFT, WINDOW_TOP},
};

pub const NUM_BRANCHES: u8 = 6;
pub const BRANCH_LEFT_X: f32 = WINDOW_LEFT + 610.0;
pub const BRANCH_RIGHT_X: f32 = WINDOW_LEFT + 1330.0;
pub const BRANCH_Y_OFFSET: f32 = 150.0;
pub const BRANCH_Z_INDEX: f32 = 6.0;

#[derive(Component)]
pub struct Branch;

#[derive(Component)]
pub struct Index(pub u8);

pub fn spawn_branches(commands: &mut Commands, assets: &Res<GameAssets>) {
    for i in 0..NUM_BRANCHES {
        commands
            .spawn(Sprite::from_image(assets.branch.clone()))
            .insert(Transform::from_xyz(-2000.0, -2000.0, BRANCH_Z_INDEX))
            .insert(ObjectSide { side: Side::None })
            .insert(Index(i))
            .insert(Branch);
    }
}

pub fn get_random_branch_side() -> Side {
    let random_int: u8 = random_range(0..=5);
    match random_int {
        0 => Side::Left,
        1 => Side::Right,
        _ => Side::None,
    }
}

pub fn get_branch_y_position(index: u8) -> f32 {
    WINDOW_TOP - (BRANCH_Y_OFFSET * index as f32)
}
