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

/// Removes actors for the current map.
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
    next_game_state.set(GameState::CleanupMap);
}

/// Initializes all actors for the current map.
pub fn initialize_actors(
    mut commands: Commands,
    query_map: Query<(&Map, &MapNumber)>,
    mut query_player_map_position: Query<&mut MapPosition, With<Player>>,
    tileset: Res<TilesetActor>,
    current_map_number: Res<CurrentMapNumber>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut current_map = None;
    for (map, map_number) in &query_map {
        if map_number.0 == current_map_number.0 {
            current_map = Some(map);
        }
    }

    let current_map = current_map.unwrap();
    initialize_rabbits(
        &mut commands,
        current_map,
        &tileset,
        current_map_number.0,
    );

    // initialize the player only if there's no player created
    let player_map_position = query_player_map_position.get_single_mut();
    if player_map_position.is_err() {
        initialize_player(
            &mut commands,
            current_map,
            &tileset,
            current_map_number.0,
        );
    } else {
        // if the player already exists, set a new spawn on the map
        let new_spawn = current_map
            .generate_random_spawning_position(vec![])
            .unwrap();
        *player_map_position.unwrap() = new_spawn;
    }
    next_game_state.set(GameState::PlayerTurn);
}
