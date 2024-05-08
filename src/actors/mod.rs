mod blob;
mod constants;
mod player;
mod rabbit;

pub use blob::*;
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
            spawn_mobs_on_current_map.run_if(in_state(AppState::InGame)),
        )
        .add_systems(
            OnEnter(GameState::CleanupActors),
            despawn_mobs_on_current_map.run_if(in_state(AppState::InGame)),
        )
        .add_systems(OnEnter(GameState::PlayerTurn), update_player_sprite)
        .add_systems(OnEnter(GameState::EnemyTurn), update_actors_sprite);
    }
}

/// Marker component for the actor entities.
#[derive(Component)]
pub struct Actor;

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
        map_position: MapPosition,
        map_number: usize,
        tileset: &TilesetActor,
        tileset_index: usize,
    ) -> Self {
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        Self {
            actor: Actor,
            map_position,
            map_number: MapNumber { 0: map_number },
            sprite: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: tileset.0.clone(),
                    index: tileset_index,
                },
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_ACTOR,
                ),
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
    query_mobs: Query<(Entity, &MapNumber), (With<Actor>, Without<Player>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    for (entity, map_number) in &query_mobs {
        if map_number.0 == current_map_number.0 {
            commands.entity(entity).despawn();
        }
    }
    next_game_state.set(GameState::CleanupMap);
}

/// Spawn mob entities (enemies, NPC...) on the current map.
pub fn spawn_mobs_on_current_map(
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
            break;
        }
    }

    if current_map.is_none() {
        panic!("no current map found with number {}", current_map_number.0);
    }

    let current_map = current_map.unwrap();

    const RABBITS_QUANTITY: usize = 3;
    const BLOB_QUANTITY: usize = 3;
    const ACTOR_QUANTIY: usize = RABBITS_QUANTITY + BLOB_QUANTITY;

    let mut actor_positions = Vec::with_capacity(ACTOR_QUANTIY);
    for _ in 0..ACTOR_QUANTIY {
        let spawn_position =
            current_map.generate_random_spawning_position(&actor_positions);
        match spawn_position {
            Ok(position) => {
                actor_positions.push(position);
            }
            Err(_) => {
                break;
            }
        }
    }

    spawn_creature::<RabbitBundle>(
        &actor_positions[0..RABBITS_QUANTITY],
        &mut commands,
        current_map_number.0,
        &tileset,
    );

    spawn_creature::<BlobBundle>(
        &actor_positions[RABBITS_QUANTITY..],
        &mut commands,
        current_map_number.0,
        &tileset,
    );

    // initialize the player only if there's no player created
    let player_map_position = query_player_map_position.get_single_mut();
    if player_map_position.is_err() {
        let player_spawn_position = match current_map
            .generate_random_spawning_position(&actor_positions)
        {
            Ok(position) => position,
            Err(_) => {
                panic!("player could not spawn");
            }
        };

        spawn_creature::<PlayerBundle>(
            &[player_spawn_position],
            &mut commands,
            current_map_number.0,
            &tileset,
        );
    } else {
        // if the player already exists, set a new spawn on the map
        let new_spawn =
            current_map.generate_random_spawning_position(&actor_positions);

        *player_map_position.unwrap() = match new_spawn {
            Ok(position) => position,
            Err(_) => {
                panic!("failed to initalize player for the first time");
            }
        };
    }
    next_game_state.set(GameState::PlayerTurn);
}

/// Represents a living creature (enemy, NPC, etc).
pub trait Creature {
    /// Retrieves the bundle for invoking the creature entity.
    fn new_bundle() -> impl Bundle;
    /// Retrieves the index on the actor tileset where the creature image is.
    fn get_tileset_index() -> usize;
}

/// Spawn creatures at specific map positions.
pub fn spawn_creature<C: Creature>(
    positions: &[MapPosition],
    commands: &mut Commands,
    current_map_number: usize,
    tileset: &TilesetActor,
) {
    for position in positions {
        commands.spawn((
            ActorBundle::new(
                *position,
                current_map_number,
                tileset,
                C::get_tileset_index(),
            ),
            C::new_bundle(),
        ));
    }
}

/// Update the sprite position of all actors of the current map according to
/// their map position.
pub fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut transform, position) = query_player
        .get_single_mut()
        .expect("multiple player found");
    let (sprite_x, sprite_y) = calculate_sprite_position(position);
    transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}

/// Update the sprite position of all actors of the current map according to
/// their map position.
pub fn update_actors_sprite(
    mut query_actors: Query<
        (&mut Transform, &MapPosition, &MapNumber),
        (With<Actor>, Without<Player>),
    >,
    current_map_number: Res<CurrentMapNumber>,
) {
    for (mut transform, map_position, map_number) in query_actors.iter_mut() {
        if map_number.0 == current_map_number.0 {
            let (sprite_x, sprite_y) = calculate_sprite_position(map_position);
            transform.translation =
                Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
        }
    }
}
