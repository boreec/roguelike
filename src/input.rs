use crate::prelude::*;
use bevy::prelude::*;

/// Checks if the player receives a directional input (i.e. an arrow key or a
/// WSQD key pressed), and moves the `Player` position accordingly.
pub fn check_player_directional_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut query_actors: Query<(&mut MapPosition, &Actor)>,
    query_map: Query<&Map>,
    input: Res<ButtonInput<KeyCode>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    let map = query_map
        .iter()
        .filter(|m| m.map_number == current_map_number.0)
        .last()
        .expect("no map found");

    let occupied_pos: Vec<MapPosition> = query_actors
        .iter()
        .filter(|(_, a)| a.map_number == current_map_number.0 && !a.is_player())
        .map(|(p, _)| *p)
        .collect();

    let (mut player_pos, _) = query_actors
        .iter_mut()
        .filter(|(_, a)| a.map_number == current_map_number.0 && a.is_player())
        .last()
        .expect("no player pos found");

    if input.any_just_pressed([KeyCode::ArrowRight, KeyCode::KeyD])
        && can_move_right(&player_pos, map, &occupied_pos)
    {
        move_right(&mut player_pos);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowLeft, KeyCode::KeyA])
        && can_move_left(&player_pos, map, &occupied_pos)
    {
        move_left(&mut player_pos);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowUp, KeyCode::KeyW])
        && can_move_up(&player_pos, map, &occupied_pos)
    {
        move_up(&mut player_pos);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::ArrowDown, KeyCode::KeyS])
        && can_move_down(&player_pos, map, &occupied_pos)
    {
        move_down(&mut player_pos);
        next_state.set(GameState::EnemyTurn);
    }
}

/// Checks if an application exit event (i.e. Escape key pressed), and moves
/// the app state to `AppState::Finished`.
pub fn check_app_exit_events(
    input: Res<ButtonInput<KeyCode>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        app_next_state.set(AppState::Finished);
        exit_events.send(bevy::app::AppExit);
    }
}
