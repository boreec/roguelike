use crate::tile::TileType;
use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct MapPosition {
    pub x: usize,
    pub y: usize,
}

impl MapPosition {
    pub fn new(x: usize, y: usize) -> Self {
        MapPosition { x, y }
    }
}

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
    pub entities: Vec<Option<Entity>>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut tiles = Vec::new();
        for i in 0..(width * height) {
            let x = i / width;
            let y = i % width;
            if y == 0 || y == height - 1 || x == 0 || x == width - 1 {
                tiles.push(TileType::GrassWithFlower)
            } else {
                tiles.push(TileType::Grass)
            }
        }
        return Map {
            width,
            height,
            tiles: tiles.clone(),
            entities: vec![None; tiles.len()],
        };
    }
}
