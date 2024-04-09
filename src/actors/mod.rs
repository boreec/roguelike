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

pub fn cleanup_actors(
    mut commands: Commands,
    query_rabbit_entities: Query<Entity, With<Rabbit>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let rabbit_entities: Vec<Entity> = query_rabbit_entities.iter().collect();
    for rabbit_entity in rabbit_entities {
        commands.entity(rabbit_entity).despawn();
    }
    next_game_state.set(GameState::InitializingMap);
}

/// Initializes all actors for a given map.
pub fn initialize_actors(
    mut commands: Commands,
    query_map: Query<&Map>,
    query_player: Query<&Player>,
    tileset: Res<TilesetActor>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let map = query_map.single();
    initialize_rabbits(&mut commands, map, &tileset);

    // initialize the player only if there's no player created
    let player = query_player.get_single();
    if player.is_err() {
        initialize_player(&mut commands, map, &tileset);
    }
    next_game_state.set(GameState::PlayerTurn);
}
