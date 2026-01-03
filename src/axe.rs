use bevy::prelude::*;

use crate::{assets::GameAssets, window::WINDOW_TOP};

pub const AXE_Z_INDEX: f32 = 9.0;
pub const AXE_Y: f32 = WINDOW_TOP - 810.0;
pub const AXE_Y_HIDDEN: f32 = -2000.0;
pub const AXE_LEFT_X: f32 = -220.0;
pub const AXE_RIGHT_X: f32 = 220.0;

#[derive(Component)]
pub struct Axe;

pub fn spawn_axe(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn(Sprite::from_image(assets.axe.clone()))
        .insert(Transform::from_xyz(AXE_RIGHT_X, AXE_Y_HIDDEN, AXE_Z_INDEX))
        .insert(Axe);
}
