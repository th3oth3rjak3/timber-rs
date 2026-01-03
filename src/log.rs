use bevy::prelude::*;

use crate::{
    assets::GameAssets,
    motion::Motion,
    player::Side,
    window::{WINDOW_BOTTOM, WINDOW_LEFT, WINDOW_RIGHT},
};

pub const LOG_SPEED_Y: f32 = 1500.0;
pub const LOG_SPEED_X_RIGHT: f32 = 5000.0;
pub const LOG_SPEED_X_LEFT: f32 = -LOG_SPEED_X_RIGHT;
pub const LOG_SPAWN_Y: f32 = WINDOW_BOTTOM + 360.0;
pub const LOG_SPAWN_X: f32 = 0.0;

#[derive(Component)]
pub struct Log;

pub fn spawn_log(commands: &mut Commands, assets: &Res<GameAssets>, z_index: f32, side: Side) {
    let x_speed = match side {
        Side::Left => LOG_SPEED_X_LEFT,
        Side::Right => LOG_SPEED_X_RIGHT,
        Side::None => 0.0,
    };

    commands
        .spawn(Sprite::from_image(assets.log.clone()))
        .insert(Transform::from_xyz(LOG_SPAWN_X, LOG_SPAWN_Y, z_index))
        .insert(Motion {
            is_active: true,
            x_speed,
            y_speed: LOG_SPEED_Y,
        })
        .insert(Log);
}

pub fn handle_log_position(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &Motion), With<Log>>,
) {
    for (entity, mut transform, motion) in query.iter_mut() {
        transform.translation.x += motion.x_speed * time.delta_secs();
        transform.translation.y += motion.y_speed * time.delta_secs();

        if transform.translation.x > WINDOW_RIGHT + 300.0
            || transform.translation.x < WINDOW_LEFT - 300.0
        {
            commands.entity(entity).despawn();
        }
    }
}
