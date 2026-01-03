use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    axe::spawn_axe,
    bee::spawn_bee,
    branch::spawn_branches,
    cloud::spawn_cloud,
    game_timer::spawn_timer_bar,
    headstone::spawn_headstone,
    player::spawn_player,
    score::{Score, spawn_score_text},
    tree::{spawn_background_trees, spawn_primary_tree},
    window::spawn_start_prompt,
};

pub fn spawn(mut commands: Commands, assets: Res<GameAssets>) {
    // Spawn the background first
    commands
        .spawn(Sprite::from_image(assets.background.clone()))
        .insert(Transform::from_xyz(0., 0., 0.));

    // Clouds go in front of the background.
    for i in 1..=3 {
        spawn_cloud(&mut commands, &assets, i as f32);
    }

    // Other Trees
    spawn_background_trees(&mut commands, &assets);

    // Log is to be hidden behind the tree at z_index 5.0
    // Branches (partially hidden behind the tree)
    spawn_branches(&mut commands, &assets);

    // Tree
    spawn_primary_tree(&mut commands, &assets);

    // Player
    spawn_player(&mut commands, &assets);

    // Headstone (same depth as player, off-screen to begin)
    spawn_headstone(&mut commands, &assets);

    // Axe
    spawn_axe(&mut commands, &assets);

    // Bee
    spawn_bee(&mut commands, &assets);

    commands.spawn(Score(0));
    spawn_score_text(&mut commands, &assets);
    spawn_start_prompt(&mut commands, &assets);
    spawn_timer_bar(&mut commands);
}
