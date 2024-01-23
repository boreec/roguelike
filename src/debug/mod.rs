mod grid;
pub use grid::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<ExecutionMode>()
            .add_systems(
                Update,
                check_execution_mode.run_if(in_state(GameState::PlayerTurn)),
            )
            .add_systems(OnEnter(ExecutionMode::Debug), show_grid)
            .add_systems(OnExit(ExecutionMode::Debug), hide_grid);
    }
}

pub fn check_execution_mode(
    mut keys: ResMut<Input<KeyCode>>,
    current_execution_mode: Res<State<ExecutionMode>>,
    mut next_execution_mode: ResMut<NextState<ExecutionMode>>,
) {
    if keys.just_pressed(KeyCode::D) {
        let mut next_state = current_execution_mode.get().clone();
        next_state.flip();
        next_execution_mode.set(next_state);
        keys.reset(KeyCode::D);
    }
}
