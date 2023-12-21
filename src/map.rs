use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileType>,
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
