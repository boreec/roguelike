mod constants;

use constants::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InitializingMap), initialize_map);
    }
}

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

/// Initialize a map by spawning tile entities depending on the map dimensions,
/// the tile placement algorithm, etc.
/// Lastly, the map entity is spawned.
fn initialize_map(
    mut commands: Commands,
    mut game_next_state: ResMut<NextState<GameState>>,
    tileset: Res<TilesetTerrain>,
) {
    // let mut ca = CellularAutomaton::new(MAP_WIDTH, MAP_HEIGHT, 0.5);
    // for _ in 0..50 {
    //     ca.transition();
    // }
    // ca.smooth();
    // let m = Map::from(ca);
    let m = Map::from((PerlinNoise::new(), MAP_WIDTH, MAP_HEIGHT));

    for (i, tile) in m.tiles.iter().enumerate() {
        let tile_position = MapPosition {
            x: i % m.width,
            y: i / m.width,
        };
        let (sprite_x, sprite_y) = calculate_sprite_position(&tile_position);
        commands.spawn(TileBundle {
            tile: Tile,
            r#type: tile.clone(),
            position: tile_position,
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_TILE,
                ),
                sprite: Sprite::default(),
                texture: tileset.1.clone(),
                atlas: TextureAtlas {
                    layout: tileset.0.clone(),
                    index: TileType::to_sprite_idx(tile),
                },
                ..Default::default()
            },
        });
    }

    commands.spawn(m);

    game_next_state.set(GameState::InitializingActors);
}

impl Map {
    /// Returns a randown position where an actor can spawn, i.e. there's no
    /// obstacle such as a rock or a tree on that position.
    pub fn generate_random_spawning_position(&self) -> MapPosition {
        let spawnable_positions: Vec<_> = self
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| tile.is_walkable())
            .map(|(index, _)| index)
            .collect();

        if spawnable_positions.is_empty() {
            panic!("There are no spawnable positions");
        }

        let mut rng = rand::thread_rng();
        let index = *spawnable_positions.choose(&mut rng).unwrap();

        MapPosition::new(index % self.width, index / self.height)
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
        Self {
            width: ca.width,
            height: ca.height,
            tiles: ca
                .cells
                .iter()
                .map(|cellular_state| match cellular_state {
                    CellularState::Alive => TileType::GrassWithStone,
                    CellularState::Dead => TileType::Grass,
                })
                .collect(),
        }
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
        let mut cells: Vec<TileType> = Vec::new();
        for i in 0..tuple.1 {
            for j in 0..tuple.2 {
                let x_scaled = i as f64 * PERLIN_NOISE_SCALE;
                let y_scaled = j as f64 * PERLIN_NOISE_SCALE;
                let noise_value = tuple.0.perlin_noise(x_scaled, y_scaled);
                if noise_value > 2.2 {
                    cells.push(TileType::GrassWithFlower)
                } else if noise_value > -0.25 {
                    cells.push(TileType::Grass);
                } else {
                    cells.push(TileType::GrassWithStone);
                }
            }
        }
        Self {
            width: tuple.1,
            height: tuple.2,
            tiles: cells,
        }
    }
}

/// Represent a position in a map.
#[derive(Component, Debug)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}
