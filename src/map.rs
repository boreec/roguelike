use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
}

impl Map {
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
    fn from(tuple: (PerlinNoise, usize, usize)) -> Self {
        let mut cells: Vec<TileType> = Vec::new();
        let scale = 0.1;
        for i in 0..tuple.1 {
            for j in 0..tuple.2 {
                let x_scaled = i as f64 * scale;
                let y_scaled = j as f64 * scale;
                let noise_value = tuple.0.perlin_noise(x_scaled, y_scaled);
                if noise_value > 0.5 {
                    cells.push(TileType::Grass);
                } else {
                    cells.push(TileType::GrassWithStone);
                }
                println!("{}", noise_value);
            }
        }
        Self {
            width: tuple.1,
            height: tuple.2,
            tiles: cells,
        }
    }
}

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
