mod constants;
mod player;
mod rabbit;

pub use constants::*;
pub use player::*;
pub use rabbit::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InitializingActors),
            initialize_rabbits.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(GameState::InitializingPlayer),
            initialize_player.run_if(in_state(AppState::InGame)),
        );
    }
}
