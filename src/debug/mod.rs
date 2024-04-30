mod constants;
mod grid;
mod tile;

use constants::*;
use grid::*;
use tile::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ExecutionMode>()
            .add_systems(
                Update,
                update_execution_mode.run_if(in_state(GameState::PlayerTurn)),
            )
            .add_systems(
                OnEnter(ExecutionMode::Debug),
                (show_grid, show_tile_coordinate_labels),
            )
            .add_systems(
                OnExit(ExecutionMode::Debug),
                (hide_grid, hide_tile_coordinate_labels),
            );
    }
}

pub fn update_execution_mode(
    mut keys: ResMut<ButtonInput<KeyCode>>,
    current_execution_mode: Res<State<ExecutionMode>>,
    mut next_execution_mode: ResMut<NextState<ExecutionMode>>,
) {
    if keys.just_pressed(DEBUG_MODE_KEY) {
        let mut next_state = current_execution_mode.get().clone();
        next_state.flip();
        next_execution_mode.set(next_state);
        keys.reset(DEBUG_MODE_KEY);
    }
}
