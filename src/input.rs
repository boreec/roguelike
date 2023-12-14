use bevy::prelude::*;

use crate::map::MapPosition;
use crate::movement::*;
use crate::player::Player;

pub fn check_player_input(
    mut query_player: Query<&mut MapPosition, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    let mut player_position = query_player.single_mut();
    if input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        if can_move_right(&player_position) {
            player_position.x += 1;
        }
    }

    if input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        if can_move_left(&player_position) {
            player_position.x -= 1;
        }
    }

    if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        if can_move_up(&player_position) {
            player_position.y -= 1;
        }
    }

    if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        if can_move_down(&player_position) {
            player_position.y += 1;
        }
    }
}

pub fn check_exit_events(
    input: Res<Input<KeyCode>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit);
    }
}
