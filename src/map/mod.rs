mod cellular_automaton;
mod constants;
mod movement;
mod noise;
mod tile;

use cellular_automaton::*;
pub use constants::*;
pub use movement::*;
use noise::*;
use tile::*;

use crate::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitializingMap), initialize_map)
            .add_systems(
                OnEnter(GameState::PlayerTurn),
                check_if_player_exit_map.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(GameState::CleanupMap), cleanup_map)
            .add_systems(
                OnEnter(GameState::EnemyTurn),
                (move_randomly, move_to_player),
            );
    }
}

/// Removes all entities (`Map`, `Tile`, etc) related to the current map.
pub fn cleanup_map(
    mut commands: Commands,
    q_map: Query<Entity, (With<Map>, With<OnDisplay>)>,
    q_tiles: Query<Entity, (With<Tile>, With<OnDisplay>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut current_map_number: ResMut<CurrentMapNumber>,
) {
    let entity = q_map.single();
    commands.entity(entity).despawn();

    for entity in &q_tiles {
        commands.entity(entity).despawn();
    }
    next_game_state.set(GameState::InitializingMap);
    current_map_number.0 += 1;
}

/// Checks if a player is on an exit tile. In that case, the game state is
/// switched to `GameState::CleanupMap`.
pub fn check_if_player_exit_map(
    q_map: Query<&Map, With<OnDisplay>>,
    q_actors: Query<(&MapPosition, &Actor), With<OnDisplay>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    let map = q_map.single();

    let (player_position, _) = q_actors
        .iter()
        .filter(|(_, actor)| actor.is_player())
        .last()
        .expect("player position not found");

    if map.exits.contains(player_position) {
        next_game_state.set(GameState::CleanupActors);
    }
}

/// Represents the environment where the actors interact together. A map is
/// made of tiles which has different properties for the actors.
#[derive(Component)]
pub struct Map {
    /// The map's width.
    pub width: usize,
    /// The map's height.
    pub height: usize,
    /// All tiles for the map, the vector index corresponds to the tile
    /// coordinates.
    pub tiles: Vec<Tile>,
    /// The exits positions for the map.
    pub exits: Vec<MapPosition>,
}

/// Initialize a map by spawning tile entities depending on the map dimensions,
/// the tile placement algorithm, etc.
/// Lastly, the map entity is spawned.
fn initialize_map(
    mut commands: Commands,
    mut game_next_state: ResMut<NextState<GameState>>,
    tileset: Res<TilesetTerrain>,
) {
    let m = if rand::thread_rng().gen_bool(0.5) {
        Map::from((PerlinNoise::new(), MAP_WIDTH, MAP_HEIGHT))
    } else {
        let mut ca = CellularAutomaton::new(MAP_WIDTH, MAP_HEIGHT, 0.5);
        for _ in 0..50 {
            ca.transition();
        }
        ca.smooth();
        Map::from(ca)
    };

    for (i, tile) in m.tiles.iter().enumerate() {
        let pos_tile = MapPosition {
            x: i % m.width,
            y: i / m.width,
        };
        commands.spawn((OnDisplay, TileBundle::new(pos_tile, &tileset, *tile)));
    }

    commands.spawn((OnDisplay, m));

    game_next_state.set(GameState::InitializingActors);
}

impl Map {
    /// Returns random positions where an actor can spawn, meaning a position
    /// with no other actors and that can be walkable.
    pub fn generate_random_positions(
        &self,
        quantity: usize,
        pos_occupied: &[MapPosition],
    ) -> Result<Vec<MapPosition>, Box<dyn std::error::Error>> {
        let mut pos_spawnable: Vec<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(index, tile)| {
                tile.is_walkable()
                    && !pos_occupied.contains(&MapPosition {
                        x: index % self.width,
                        y: index / self.width,
                    })
            })
            .map(|(index, _)| MapPosition {
                x: index % self.width,
                y: index / self.width,
            })
            .collect();

        if pos_spawnable.is_empty() {
            return Err("no spawnable positions".into());
        }

        let mut rng = rand::thread_rng();
        pos_spawnable.shuffle(&mut rng);

        Ok(pos_spawnable[0..quantity].to_vec())
    }

    /// Adds an exit tile on the right side of the map. The position is
    /// selected randomly.
    pub fn add_exit_tile(&mut self) {
        let pos_spawnable: Vec<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(index, tile)| {
                tile.is_walkable() && *index % self.width == self.width - 1
            })
            .map(|(index, _)| index)
            .collect();

        assert!(
            !pos_spawnable.is_empty(),
            "mo positions available for exit tile",
        );

        let mut rng = rand::thread_rng();
        let index = *pos_spawnable.choose(&mut rng).unwrap();

        self.tiles[index].kind = TileKind::LevelExit;

        let pos_exit = MapPosition {
            x: index % self.width,
            y: index / self.width,
        };

        self.exits.push(pos_exit);
    }

    /// Converts a given `MapPosition` to an index in the tiles vector.
    pub fn as_tile_index(&self, pos: &MapPosition) -> Result<usize, String> {
        let index = pos.x + pos.y * self.width;
        if index >= self.tiles.len() {
            return Err("index out of bounds".into());
        }
        Ok(index)
    }

    /// Moves an actor from a tile to another and updates its map position.
    pub fn move_actor(
        &mut self,
        pos_actor_old: &mut MapPosition,
        pos_actor_new: &MapPosition,
    ) -> Result<(), String> {
        let index_from = self.as_tile_index(pos_actor_old)?;
        let index_to = self.as_tile_index(pos_actor_new)?;
        self.tiles[index_to].actor = self.tiles[index_from].actor.take();
        *pos_actor_old = *pos_actor_new;
        Ok(())
    }
}

impl From<CellularAutomaton> for Map {
    /// Constructs a `Map` using a cellular automaton.
    ///
    /// # Arguments
    ///
    /// - `ca`: A `CellularAutomaton` initialized struct.
    ///
    /// # Returns
    ///
    /// A `Map` where the tiles are determined by the cellular automaton state.
    fn from(ca: CellularAutomaton) -> Self {
        let mut map = Self {
            width: ca.width,
            height: ca.height,
            tiles: ca
                .cells
                .iter()
                .map(|cellular_state| {
                    let tile_kind = match cellular_state {
                        CellularState::Alive => {
                            let mut rng = rand::thread_rng();
                            if rng.gen_bool(PROBABILITY_STONE_DAMAGED) {
                                TileKind::GrassWithStoneDamaged
                            } else {
                                TileKind::GrassWithStone
                            }
                        }
                        CellularState::Dead => TileKind::Grass,
                    };
                    Tile::from_kind(tile_kind)
                })
                .collect(),
            exits: vec![],
        };

        map.add_exit_tile();
        map
    }
}

impl From<(PerlinNoise, usize, usize)> for Map {
    /// Constructs a `Map` using Perlin noise.
    ///
    /// # Arguments
    ///
    /// - `tuple`: A tuple with three parameters representing a `PerlinNoise` struct,
    ///            the width, and the height of the map.
    ///
    /// # Returns
    ///
    /// A `Map` where the tiles are determined by Perlin noise.
    fn from(tuple: (PerlinNoise, usize, usize)) -> Self {
        let mut cells: Vec<Tile> = Vec::new();
        for i in 0..tuple.1 {
            for j in 0..tuple.2 {
                let x_scaled = i as f64 * PERLIN_NOISE_SCALE;
                let y_scaled = j as f64 * PERLIN_NOISE_SCALE;
                let noise_value = tuple.0.perlin_noise(x_scaled, y_scaled);

                let kind = if noise_value > 2.2 {
                    TileKind::GrassWithFlower
                } else if noise_value > -0.25 {
                    TileKind::Grass
                } else {
                    let mut rng = rand::thread_rng();
                    if rng.gen_bool(PROBABILITY_STONE_DAMAGED) {
                        TileKind::GrassWithStoneDamaged
                    } else {
                        TileKind::GrassWithStone
                    }
                };
                cells.push(Tile::from_kind(kind));
            }
        }

        let mut map = Self {
            width: tuple.1,
            height: tuple.2,
            tiles: cells,
            exits: vec![],
        };

        map.add_exit_tile();
        map
    }
}

/// Represents a position in a `Map`.
#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub fn as_sprite_coordinates(&self) -> (f32, f32) {
        (
            (self.x as f32).mul_add(SPRITE_TILE_WIDTH, SPRITE_TILE_WIDTH / 2.0),
            (-1f32 * self.y as f32)
                .mul_add(SPRITE_TILE_HEIGHT, -(SPRITE_TILE_HEIGHT / 2.0)),
        )
    }

    /// Returns a `MapPostion` on the left of the current one.
    pub fn left(&self) -> Result<Self, String> {
        if self.x == 0 {
            return Err("left can't be out of bounds".into());
        }
        Ok(Self::new(self.x - 1, self.y))
    }

    /// Returns a `MapPostion` on the right of the current one.
    pub fn right(&self) -> Result<Self, String> {
        Ok(Self::new(self.x + 1, self.y))
    }

    /// Returns a `MapPostion` above the current one.
    pub fn up(&self) -> Result<Self, String> {
        if self.y == 0 {
            return Err("up can't be out of bounds".into());
        }
        Ok(Self::new(self.x, self.y - 1))
    }

    /// Returns a `MapPostion` under the current one.
    pub fn down(&self) -> Result<Self, String> {
        Ok(Self::new(self.x, self.y + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_random_spawning_position_success() {
        let map1x1 = Map {
            width: 1,
            height: 1,
            tiles: vec![Tile::default()],
            exits: vec![],
        };

        let spawn = map1x1.generate_random_positions(1, &vec![]);

        assert!(spawn.is_ok());
        assert_eq!(vec![MapPosition::new(0, 0)], spawn.unwrap());
    }

    #[test]
    fn test_generate_random_spawning_position_failure() {
        let mut map1x1 = Map {
            width: 1,
            height: 1,
            tiles: vec![Tile::from_kind(TileKind::GrassWithStone)],
            exits: vec![],
        };

        let spawn = map1x1.generate_random_positions(1, &vec![]);
        assert!(spawn.is_err());

        map1x1.tiles = vec![Tile::default()];

        let spawn = map1x1.generate_random_positions(1, &vec![]);
        assert!(spawn.is_ok());

        let spawn = map1x1
            .generate_random_positions(1, &vec![MapPosition { x: 0, y: 0 }]);
        assert!(spawn.is_err());
    }
}
