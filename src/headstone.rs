use bevy::prelude::*;

use crate::assets::GameAssets;

pub const HEADSTONE_Y_HIDDEN: f32 = -2000.0;

#[derive(Component)]
pub struct Headstone;

pub fn spawn_headstone(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn(Sprite::from_image(assets.headstone.clone()))
        .insert(Transform::from_xyz(0., HEADSTONE_Y_HIDDEN, 8.0))
        .insert(Headstone);
}
