use crate::assets::GameAssets;
use crate::motion::Motion;
use crate::window::*;
use bevy::prelude::*;
use rand::random_range;

pub const BEE_MIN_Y: f32 = WINDOW_BOTTOM + 50.0;
pub const BEE_MAX_Y: f32 = WINDOW_BOTTOM + 500.0;
pub const BEE_SPEED_MIN: f32 = 200.0;
pub const BEE_SPEED_MAX: f32 = 399.0;
pub const BEE_Z_INDEX: f32 = 10.0;

#[derive(Component)]
pub struct Bee;

/// Spawns a bee that travels across the screen
pub fn spawn_bee(commands: &mut Commands, assets: &Res<GameAssets>) {
    let x_position = random_range(WINDOW_LEFT..=WINDOW_RIGHT);
    let y_position = get_random_bee_y_position();

    commands
        .spawn(Sprite::from_image(assets.bee.clone()))
        .insert(Transform::from_xyz(x_position, y_position, BEE_Z_INDEX))
        .insert(Motion {
            is_active: true,
            x_speed: get_random_bee_speed(),
            y_speed: 0.0,
        })
        .insert(Bee);
}

pub fn update_bee_position(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &mut Motion), With<Bee>>,
) {
    for (mut transform, mut motion) in query.iter_mut() {
        if !motion.is_active {
            continue;
        }

        transform.translation.x += motion.x_speed * time.delta_secs();

        // If cloud leaves the screen, respawn it
        if transform.translation.x < WINDOW_LEFT - 100.0 {
            transform.translation.x = WINDOW_RIGHT + 100.0;
            transform.translation.y = get_random_bee_y_position();
            motion.x_speed = get_random_bee_speed();
        }
    }
}

pub fn get_random_bee_speed() -> f32 {
    let bee_speed = random_range(BEE_SPEED_MIN..=BEE_SPEED_MAX);
    bee_speed * -1.0
}

pub fn get_random_bee_y_position() -> f32 {
    random_range(BEE_MIN_Y..=BEE_MAX_Y)
}
