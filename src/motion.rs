use bevy::prelude::*;

#[derive(Component)]
pub struct Motion {
    pub is_active: bool,
    pub x_speed: f32,
    pub y_speed: f32,
}
