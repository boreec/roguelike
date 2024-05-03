use crate::prelude::*;
use bevy::prelude::*;

/// Checks for the player inputs and moves the character accordingly.
pub fn check_player_directional_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut query_player: Query<&mut MapPosition, With<Player>>,
    query_other_actors: Query<&MapPosition, (With<Actor>, Without<Player>)>,
    query_map: Query<&Map>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut player_position = query_player.single_mut();
    let map = query_map.single();

    let mut occupied_positions: Vec<MapPosition> = vec![];
    for other_actor_position in &query_other_actors {
        occupied_positions.push(*other_actor_position);
    }

    if input.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD])
        && can_move_right(&player_position, map, &occupied_positions)
    {
        move_right(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA])
        && can_move_left(&player_position, map, &occupied_positions)
    {
        move_left(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW])
        && can_move_up(&player_position, map, &occupied_positions)
    {
        move_up(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS])
        && can_move_down(&player_position, map, &occupied_positions)
    {
        move_down(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }
}

pub fn check_exit_events(
    input: Res<ButtonInput<KeyCode>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_next_state.set(AppState::Finished);
        exit_events.send(bevy::app::AppExit);
    }
}
