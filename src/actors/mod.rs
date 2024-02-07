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
            initialize_actors.run_if(in_state(AppState::InGame)),
        );
    }
}

pub fn initialize_actors(
    mut commands: Commands,
    query_map: Query<&Map>,
    tileset: Res<TilesetActor>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let map = query_map.single();
    initialize_rabbits(&mut commands, map, &tileset);
    initialize_player(&mut commands, map, &tileset);
    next_game_state.set(GameState::PlayerTurn);
}
