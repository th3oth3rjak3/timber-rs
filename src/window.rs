pub const WINDOW_LEFT: f32 = (1920.0 / 2.0) * -1.0;
pub const WINDOW_RIGHT: f32 = 1920.0 / 2.0;
pub const WINDOW_TOP: f32 = 1080.0 / 2.0;
pub const WINDOW_BOTTOM: f32 = (1080.0 / 2.0) * -1.0;

use crate::{
    assets::GameAssets,
    events::{OutOfTime, StartGame},
};
use bevy::prelude::*;

#[derive(Component)]
pub struct EnterPrompt;

pub fn spawn_start_prompt(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            EnterPrompt,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Press Enter to start!"),
                        TextFont {
                            font: assets.default_font.clone(),
                            font_size: 75.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn spawn_you_died_prompt(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            EnterPrompt,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("You died! Press Enter to start again!"),
                        TextFont {
                            font: assets.default_font.clone(),
                            font_size: 75.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn hide_start_prompt(
    _: On<StartGame>,
    mut commands: Commands,
    mut start_prompt_query: Query<Entity, With<EnterPrompt>>,
) {
    let entity = start_prompt_query.single_mut().unwrap();
    commands.entity(entity).despawn();
}

pub fn show_out_of_time_prompt(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Flex,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            EnterPrompt,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        padding: UiRect::all(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Out of time! Press Enter to play again!!"),
                        TextFont {
                            font: assets.default_font.clone(),
                            font_size: 75.0,
                            ..Default::default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

pub fn handle_spawn_out_of_time_message(
    _: On<OutOfTime>,
    mut commands: Commands,
    assets: Res<GameAssets>,
) {
    show_out_of_time_prompt(&mut commands, &assets);
}
