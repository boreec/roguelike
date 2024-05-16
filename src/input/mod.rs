mod constants;

use crate::prelude::*;
use bevy::input::mouse::MouseWheel;
use constants::*;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                check_camera_zoom_via_mouse,
                check_player_move_via_keys,
                check_player_skip_turn_via_keys,
            )
                .run_if(in_state(GameState::PlayerTurn)),
        )
        .add_systems(
            Update,
            check_app_exit_via_keys.run_if(in_state(AppState::InGame)),
        );
    }
}

/// Updates the camera zoom depending on the mouse wheel input.
pub fn check_camera_zoom_via_mouse(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q_main_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut projection = q_main_camera.single_mut();
    let mut log_scale = projection.scale.ln();
    for ev in scroll_evr.read() {
        if ev.unit != MouseScrollUnit::Line {
            continue;
        }
        if ev.y > 0f32 && projection.scale > CAMERA_ZOOM_IN_MAX {
            log_scale -= CAMERA_ZOOM_INCREMENT;
        } else if ev.y < 0f32 && projection.scale < CAMERA_ZOOM_OUT_MAX {
            log_scale += CAMERA_ZOOM_INCREMENT;
        }
    }
    projection.scale = log_scale.exp();
}

/// Checks if the player skip turn when `KEY_PLAYER_SKIP_TURN` is pressed.
pub fn check_player_skip_turn_via_keys(
    mut next_state: ResMut<NextState<GameState>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KEY_PLAYER_SKIP_TURN) {
        next_state.set(GameState::EnemyTurn);
    }
}

/// Checks if the player receives a directional input (i.e. an arrow key or a
/// WSQD key pressed), and moves the `Player` position accordingly.
pub fn check_player_move_via_keys(
    mut next_state: ResMut<NextState<GameState>>,
    mut q_actors: Query<(&mut MapPosition, &Actor), With<OnDisplay>>,
    mut q_map: Query<&mut Map, With<OnDisplay>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let mut map = q_map.single_mut();

    let (mut pos_player, _) = q_actors
        .iter_mut()
        .filter(|(_, a)| a.is_player())
        .last()
        .expect("no player pos found");

    let pos_player_old = pos_player.clone();

    if input.any_just_pressed(KEYS_PLAYER_MOVE_RIGHT)
        && can_move_right(&pos_player.clone(), &map)
    {
        move_right(&mut pos_player);
    }

    if input.any_just_pressed(KEYS_PLAYER_MOVE_LEFT)
        && can_move_left(&pos_player.clone(), &map)
    {
        move_left(&mut pos_player);
    }

    if input.any_just_pressed(KEYS_PLAYER_MOVE_UP)
        && can_move_up(&pos_player.clone(), &map)
    {
        move_up(&mut pos_player);
    }

    if input.any_just_pressed(KEYS_PLAYER_MOVE_DOWN)
        && can_move_down(&pos_player.clone(), &map)
    {
        move_down(&mut pos_player);
    }

    if pos_player_old != pos_player.clone() {
        let tile_pos_old = map.as_tile_index(&pos_player_old).unwrap();
        let tile_pos_new = map.as_tile_index(&pos_player.clone()).unwrap();
        map.tiles[tile_pos_new].actor = map.tiles[tile_pos_old].actor.take();
        next_state.set(GameState::EnemyTurn);
    }
}

/// Checks if an application exit event (i.e. Escape key pressed), and moves
/// the app state to `AppState::Finished`.
pub fn check_app_exit_via_keys(
    input: Res<ButtonInput<KeyCode>>,
    mut app_next_state: ResMut<NextState<AppState>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KEY_APP_EXIT) {
        app_next_state.set(AppState::Finished);
        exit_events.send(bevy::app::AppExit);
    }
}
