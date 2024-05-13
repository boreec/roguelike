mod constants;

pub use constants::*;

use crate::prelude::*;
use bevy::prelude::*;
use std::collections::HashMap;

pub struct ActorsPlugin;

impl Plugin for ActorsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::InitializingActors),
            spawn_mobs_on_current_map.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(GameState::CleanupActors),
            despawn_mobs_on_current_map.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(GameState::PlayerTurn), update_actor_sprites)
        .add_systems(OnEnter(GameState::EnemyTurn), update_actor_sprites);
    }
}

#[derive(Clone, Component, Copy)]
pub struct Actor {
    pub kind: ActorKind,
}

impl Actor {
    pub fn get_tileset_index(&self) -> usize {
        match self.kind {
            ActorKind::Blob => TILESET_ACTOR_IDX_BLOB,
            ActorKind::Player => TILESET_ACTOR_IDX_PLAYER,
            ActorKind::Rabbit => TILESET_ACTOR_IDX_RABBIT,
        }
    }

    pub fn is_player(&self) -> bool {
        self.kind == ActorKind::Player
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ActorKind {
    Blob,
    Rabbit,
    Player,
}

/// Bundle for spawning actor entities.
#[derive(Bundle)]
pub struct ActorBundle {
    /// Marker component for actor entities.
    pub actor: Actor,
    /// The map where the actor is at.
    pub map_number: MapNumber,
    /// The map's position where the actor is at.
    pub map_position: MapPosition,
    /// The sprite representing the actor.
    pub sprite: SpriteSheetBundle,
}

impl ActorBundle {
    pub fn new(
        actor: Actor,
        map_position: MapPosition,
        map_number: usize,
        tileset: &TilesetActor,
    ) -> Self {
        let (x, y) = map_position.as_sprite_coordinates();
        Self {
            actor: actor.clone(),
            map_position,
            map_number: MapNumber { 0: map_number },
            sprite: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: tileset.0.clone(),
                    index: actor.get_tileset_index(),
                },
                transform: Transform::from_xyz(x, y, Z_INDEX_ACTOR),
                texture: tileset.1.clone(),
                sprite: Sprite::default(),
                ..Default::default()
            },
        }
    }
}
/// Despawn mob entities on the current map.
pub fn despawn_mobs_on_current_map(
    mut commands: Commands,
    query_actors: Query<(Entity, &MapNumber), With<Actor>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    for (entity, map_number) in &query_actors {
        if map_number.0 == current_map_number.0 {
            commands.entity(entity).despawn();
        }
    }
    next_game_state.set(GameState::CleanupMap);
}

pub fn generate_spawn_counts(
    _map: &Map,
    _map_number: &MapNumber,
) -> HashMap<ActorKind, usize> {
    let mut result = HashMap::new();
    result.insert(ActorKind::Blob, 3);
    result.insert(ActorKind::Rabbit, 3);
    return result;
}

/// Spawn mob entities (enemies, NPC...) on the current map.
pub fn spawn_mobs_on_current_map(
    mut commands: Commands,
    query_map: Query<(&Map, &MapNumber)>,
    mut query_actors: Query<(&mut MapPosition, &MapNumber, &Actor)>,
    tileset: Res<TilesetActor>,
    current_map_number: Res<CurrentMapNumber>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let (map, map_number) = query_map
        .iter()
        .filter(|(_, map_n)| map_n.0 == current_map_number.0)
        .last()
        .expect(
            format!(
                "a map should be associated with the current map number: {}",
                current_map_number.0
            )
            .as_str(),
        );

    let pos_occupied: Vec<MapPosition> = query_actors
        .iter()
        .filter(|(_, m_n, _)| m_n.0 == current_map_number.0)
        .map(|(m_p, _, _)| *m_p)
        .collect();

    let spawn_counts = generate_spawn_counts(map, map_number);
    let actor_quantity = spawn_counts.values().fold(0, |acc, &x| acc + x);
    let pos_actors = map
        .generate_random_positions(actor_quantity, &pos_occupied)
        .unwrap();

    let mut spawned_quantity = 0;
    for (actor_kind, quantity) in spawn_counts.iter() {
        spawn_creature(
            *actor_kind,
            &pos_actors[spawned_quantity..spawned_quantity + quantity],
            &mut commands,
            current_map_number.0,
            &tileset,
        );
        spawned_quantity += quantity;
    }

    // initialize the player only if there's no player created
    let pos_player = query_actors
        .iter_mut()
        .filter(|(_, map_n, actor)| {
            map_n.0 == current_map_number.0 && actor.is_player()
        })
        .last();

    // if the player already exists, set a new spawn on the map
    if let Some(mut pos_player) = pos_player {
        let pos_new_spawn = map
            .generate_random_positions(1, &pos_actors)
            .expect("failed to initialize player spawn")
            .pop()
            .unwrap();

        pos_player.0.x = pos_new_spawn.x;
        pos_player.0.y = pos_new_spawn.y;
    } else {
        let pos_player_spawn = map
            .generate_random_positions(1, &pos_actors)
            .unwrap()
            .last()
            .unwrap()
            .clone();

        spawn_creature(
            ActorKind::Player,
            &[pos_player_spawn],
            &mut commands,
            current_map_number.0,
            &tileset,
        );
    }
    next_game_state.set(GameState::PlayerTurn);
}

/// Spawn creatures at specific map positions.
pub fn spawn_creature(
    actor_kind: ActorKind,
    positions: &[MapPosition],
    commands: &mut Commands,
    current_map_number: usize,
    tileset: &TilesetActor,
) {
    for position in positions {
        commands.spawn(ActorBundle::new(
            Actor { kind: actor_kind },
            *position,
            current_map_number,
            tileset,
        ));
    }
}

/// Update the sprite position of all actors of the current map according to
/// their map position.
pub fn update_actor_sprites(
    mut query_actors: Query<
        (&mut Transform, &MapPosition, &MapNumber),
        With<Actor>,
    >,
    current_map_number: Res<CurrentMapNumber>,
) {
    for (mut transform, pos, map_number) in &mut query_actors {
        if map_number.0 == current_map_number.0 {
            let (x, y) = pos.as_sprite_coordinates();
            transform.translation = Vec3::new(x, y, Z_INDEX_ACTOR);
        }
    }
}
