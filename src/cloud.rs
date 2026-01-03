use crate::assets::GameAssets;
use crate::motion::Motion;
use crate::window::*;
use bevy::prelude::*;
use rand::random_range;

pub const CLOUD_MIN_Y: f32 = WINDOW_TOP - 200.0;
pub const CLOUD_MAX_Y: f32 = WINDOW_TOP - 50.0;
pub const CLOUD_SPEED_MIN: f32 = 25.0;
pub const CLOUD_SPEED_MAX: f32 = 100.0;

#[derive(Component)]
pub struct Cloud;

/// Spawns a cloud at a random location.
pub fn spawn_cloud(commands: &mut Commands, assets: &Res<GameAssets>, z_index: f32) {
    let x_position = random_range(WINDOW_LEFT..=WINDOW_RIGHT);
    let y_position = random_range(CLOUD_MIN_Y..=CLOUD_MAX_Y);
    let x_speed = random_range(CLOUD_SPEED_MIN..=CLOUD_SPEED_MAX);

    commands
        .spawn(Sprite::from_image(assets.cloud.clone()))
        .insert(Transform::from_xyz(x_position, y_position, z_index))
        .insert(Motion {
            is_active: true,
            x_speed,
            y_speed: 0.0,
        })
        .insert(Cloud);
}

pub fn update_cloud_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Motion), With<Cloud>>,
) {
    for (mut transform, mut motion) in query.iter_mut() {
        if !motion.is_active {
            continue;
        }

        transform.translation.x += motion.x_speed * time.delta_secs();

        // If cloud leaves the screen, respawn it
        if transform.translation.x > WINDOW_RIGHT + 200.0 {
            transform.translation.x = WINDOW_LEFT - 200.0;
            transform.translation.y = random_range(CLOUD_MIN_Y..=CLOUD_MAX_Y);
            motion.x_speed = random_range(CLOUD_SPEED_MIN..=CLOUD_SPEED_MAX);
        }
    }
}
