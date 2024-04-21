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
        )
        .add_systems(OnEnter(GameState::CleanupActors), cleanup_actors);
    }
}

/// Removes actors for a given map.
pub fn cleanup_actors(
    mut commands: Commands,
    query: Query<(Entity, &MapNumber)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    for (entity, map_number) in &query {
        if map_number.0 == current_map_number.0 {
            commands.entity(entity).despawn();
        }
    }
    next_game_state.set(GameState::InitializingMap);
}

/// Initializes all actors for a given map.
pub fn initialize_actors(
    mut commands: Commands,
    query_map: Query<&Map>,
    mut query_player_map_position: Query<&mut MapPosition, With<Player>>,
    tileset: Res<TilesetActor>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let map = query_map.single();
    initialize_rabbits(&mut commands, map, &tileset);

    // initialize the player only if there's no player created
    let player_map_position = query_player_map_position.get_single_mut();
    if player_map_position.is_err() {
        initialize_player(&mut commands, map, &tileset);
    } else {
        let new_spawn = map.generate_random_spawning_position();
        *player_map_position.unwrap() = new_spawn;
    }
    next_game_state.set(GameState::PlayerTurn);
}
