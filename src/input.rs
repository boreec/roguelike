use bevy::prelude::*;

use crate::map::*;
use crate::movement::*;
use crate::player::Player;

pub fn check_player_input(
    mut query_player: Query<&mut MapPosition, With<Player>>,
    query_map: Query<&Map>,
    input: Res<Input<KeyCode>>,
) {
    let mut player_position = query_player.single_mut();
    let map = query_map.single();

    if input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
        if can_move_right(&player_position, &map) {
            move_right(&mut player_position)
        }
    }

    if input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
        if can_move_left(&player_position) {
            move_left(&mut player_position)
        }
    }

    if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
        if can_move_up(&player_position) {
            move_up(&mut player_position);
        }
    }

    if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
        if can_move_down(&player_position, &map) {
            move_down(&mut player_position);
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
