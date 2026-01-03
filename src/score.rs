use bevy::prelude::*;

use crate::{assets::GameAssets, events::StartGame};

#[derive(Component)]
pub struct Score(pub u32);

#[derive(Component)]
pub struct ScoreText;

pub fn handle_score_reset(_: On<StartGame>, mut score_query: Query<&mut Score>) {
    for mut score in score_query.iter_mut() {
        score.0 = 0;
    }
}

pub fn spawn_score_text(commands: &mut Commands, assets: &Res<GameAssets>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(20.0),
                left: Val::Px(20.0),

                // Padding around the text
                padding: UiRect::all(Val::Px(10.0)),

                // Optional: rounded corners sizing
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.8)), // translucent black
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Score: 0"),
                TextFont {
                    font: assets.default_font.clone(),
                    font_size: 48.0,
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });
}

pub fn update_score_text(score: Query<&Score>, mut text_query: Query<&mut Text, With<ScoreText>>) {
    let score = score.single().unwrap();

    for mut text in &mut text_query {
        text.0 = format!("Score: {}", score.0);
    }
}
