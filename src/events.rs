use bevy::{
    audio::{PlaybackMode, Volume},
    prelude::*,
};

use crate::assets::GameAssets;

#[derive(Event)]
pub struct StartGame;

#[derive(Event)]
pub struct OutOfTime;

pub fn handle_out_of_time_sound(_: On<OutOfTime>, mut commands: Commands, assets: Res<GameAssets>) {
    commands.spawn((
        AudioPlayer::new(assets.out_of_time.clone()),
        PlaybackSettings {
            mode: PlaybackMode::Despawn,
            volume: Volume::Linear(0.4),
            ..Default::default()
        },
    ));
}
