mod grid;
pub use grid::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            display_grid.run_if(in_state(AppState::InGame)),
        );
    }
}
