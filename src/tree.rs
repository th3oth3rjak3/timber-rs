use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    window::{WINDOW_LEFT, WINDOW_TOP},
};

pub fn spawn_primary_tree(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn(Sprite::from_image(assets.tree.clone()))
        .insert(Transform::from_xyz(0., WINDOW_TOP - 445., 7.0));
}

pub fn spawn_background_trees(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn(Sprite::from_image(assets.tree_alt.clone()))
        .insert(Transform::from_xyz(
            WINDOW_LEFT + 300.0,
            WINDOW_TOP - 350.0,
            4.,
        ));

    commands
        .spawn(Sprite::from_image(assets.tree_alt.clone()))
        .insert(Transform::from_xyz(
            WINDOW_LEFT + 1500.0,
            WINDOW_TOP - 360.0,
            4.,
        ));

    commands
        .spawn(Sprite::from_image(assets.tree_alt.clone()))
        .insert(Transform::from_xyz(
            WINDOW_LEFT + 2000.0,
            WINDOW_TOP - 400.0,
            4.,
        ));
}
