use crate::prelude::*;
use bevy::prelude::*;

pub fn check_player_input(
    mut next_state: ResMut<NextState<GameState>>,
    mut query_player: Query<&mut MapPosition, With<Player>>,
    query_map: Query<&Map>,
    input: Res<Input<KeyCode>>,
) {
    let mut player_position = query_player.single_mut();
    let map = query_map.single();

    if input.any_just_pressed([KeyCode::Right, KeyCode::D])
        && can_move_right(&player_position, &map)
    {
        move_right(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::Left, KeyCode::A])
        && can_move_left(&player_position, &map)
    {
        move_left(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::Up, KeyCode::W])
        && can_move_up(&player_position, &map)
    {
        move_up(&mut player_position);
        next_state.set(GameState::EnemyTurn);
    }

    if input.any_just_pressed([KeyCode::Down, KeyCode::S])
        && can_move_down(&player_position, &map)
    {
        move_down(&mut player_position);
        next_state.set(GameState::EnemyTurn);
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
