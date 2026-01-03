use bevy::{prelude::*, window::WindowResolution};

use crate::{
    game_timer::{GameTimer, tick_game_timer},
    input::GameState,
};

mod assets;
mod axe;
mod bee;
mod branch;
mod camera;
mod chop_system;
mod cloud;
mod events;
mod game_timer;
mod headstone;
mod input;
mod log;
mod motion;
mod player;
mod score;
mod sprites;
mod tree;
mod window;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Timber!!".to_string(),
                resolution: WindowResolution::new(1920, 1080),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(GameTimer::new(6.0))
        .insert_state(GameState::Paused)
        .add_systems(
            Startup,
            (camera::spawn, (assets::load, sprites::spawn).chain()),
        )
        .add_systems(
            Update,
            (
                cloud::update_cloud_position,
                bee::update_bee_position,
                input::handle_player_input,
                log::handle_log_position,
                score::update_score_text,
                game_timer::update_timer_bar,
                tick_game_timer.run_if(in_state(GameState::Playing)),
            ),
        )
        .add_observer(chop_system::play_chop_sound)
        .add_observer(chop_system::increase_player_score)
        .add_observer(chop_system::handle_player_branch_interaction)
        .add_observer(chop_system::handle_log_active)
        .add_observer(chop_system::show_axe_sprite)
        .add_observer(game_timer::add_time_on_chop)
        .add_observer(player::handle_death_sound)
        .add_observer(player::handle_death_player_translation)
        .add_observer(player::handle_game_state_paused)
        .add_observer(player::handle_hide_axe_on_death)
        .add_observer(player::handle_spawn_death_message)
        .add_observer(input::handle_start_game)
        .add_observer(input::handle_reset_player_sprite)
        .add_observer(input::handle_reset_headstone_sprite)
        .add_observer(input::handle_reset_branches)
        .add_observer(score::handle_score_reset)
        .add_observer(window::hide_start_prompt)
        .add_observer(window::handle_spawn_out_of_time_message)
        .add_observer(game_timer::reset_game_timer)
        .add_observer(events::handle_out_of_time_sound)
        .run();
}
