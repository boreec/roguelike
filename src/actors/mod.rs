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
    pub hostility: ActorHostility,
}

impl Actor {
    pub fn new(kind: ActorKind) -> Self {
        Self {
            kind,
            hostility: kind.get_hostility(),
        }
    }

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

    pub fn is_hostile(&self) -> bool {
        self.hostility == ActorHostility::Enemy
    }

    pub fn is_friendly(&self) -> bool {
        self.hostility == ActorHostility::Friend
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub enum ActorKind {
    Blob,
    Rabbit,
    Player,
}

impl ActorKind {
    pub fn get_hostility(&self) -> ActorHostility {
        match self {
            ActorKind::Blob => ActorHostility::Enemy,
            ActorKind::Rabbit => ActorHostility::Friend,
            ActorKind::Player => ActorHostility::Friend,
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ActorHostility {
    Enemy,
    Friend,
}

/// Bundle for spawning actor entities.
#[derive(Bundle)]
pub struct ActorBundle {
    /// Marker component for actor entities.
    pub actor: Actor,
    /// The map's position where the actor is at.
    pub map_position: MapPosition,
    /// The sprite representing the actor.
    pub sprite: SpriteSheetBundle,
}

impl ActorBundle {
    pub fn new(
        actor: Actor,
        map_position: MapPosition,
        tileset: &TilesetActor,
    ) -> Self {
        let (x, y) = map_position.as_sprite_coordinates();
        Self {
            actor: actor.clone(),
            map_position,
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
    q_actors: Query<(Entity, &Actor), With<OnDisplay>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (entity, actor) in &q_actors {
        if actor.is_player() {
            continue;
        }
        commands.entity(entity).despawn();
    }
    next_game_state.set(GameState::CleanupMap);
}

pub fn generate_spawn_counts(_map: &Map) -> HashMap<ActorKind, usize> {
    let mut result = HashMap::new();
    result.insert(ActorKind::Blob, 3);
    result.insert(ActorKind::Rabbit, 3);
    return result;
}

/// Spawn mob entities (enemies, NPC...) on the current map.
pub fn spawn_mobs_on_current_map(
    mut commands: Commands,
    mut q_map: Query<&mut Map, With<OnDisplay>>,
    mut q_actors: Query<(&mut MapPosition, &Actor), With<OnDisplay>>,
    tileset: Res<TilesetActor>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let mut map = q_map.single_mut();

    let pos_occupied: Vec<MapPosition> =
        q_actors.iter().map(|(m_p, _)| *m_p).collect();

    let spawn_counts = generate_spawn_counts(&map);
    let actor_quantity = spawn_counts.values().fold(0, |acc, &x| acc + x);
    let pos_actors = map
        .generate_random_positions(actor_quantity, &pos_occupied)
        .unwrap();

    let mut spawned_quantity = 0;
    for (actor_kind, quantity) in spawn_counts.iter() {
        spawn_creature(
            *actor_kind,
            &mut map,
            &pos_actors[spawned_quantity..spawned_quantity + quantity],
            &mut commands,
            &tileset,
        );
        spawned_quantity += quantity;
    }

    // initialize the player only if there's no player created
    let pos_player = q_actors.iter_mut().filter(|(_, a)| a.is_player()).last();

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
            &mut map,
            &[pos_player_spawn],
            &mut commands,
            &tileset,
        );
    }
    next_game_state.set(GameState::PlayerTurn);
}

/// Spawn creatures at specific map positions.
pub fn spawn_creature(
    actor_kind: ActorKind,
    map: &mut Map,
    positions: &[MapPosition],
    commands: &mut Commands,
    tileset: &TilesetActor,
) -> Result<(), String> {
    for position in positions {
        let tile_pos = map.as_tile_index(position).unwrap();
        if map.tiles[tile_pos].actor.is_some() {
            return Err("tile already occupied".into());
        }
        let actor = Actor::new(actor_kind);
        map.tiles[tile_pos].actor = Some(actor);
        commands
            .spawn((OnDisplay, ActorBundle::new(actor, *position, tileset)));
    }
    Ok(())
}

/// Update the sprite position of all actors of the current map according to
/// their map position.
pub fn update_actor_sprites(
    mut q_actors: Query<
        (&mut Transform, &MapPosition),
        (With<OnDisplay>, With<Actor>),
    >,
) {
    for (mut transform, pos) in &mut q_actors {
        let (x, y) = pos.as_sprite_coordinates();
        transform.translation = Vec3::new(x, y, Z_INDEX_ACTOR);
    }
}
