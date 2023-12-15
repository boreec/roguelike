use bevy::prelude::Bundle;
use bevy::prelude::Component;

#[derive(Bundle)]
pub struct MapBundle {
    pub map: Map,
    pub size: MapSize,
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

#[derive(Component)]
pub struct Map;

#[derive(Component)]
pub struct MapSize {
    pub width: usize,
    pub height: usize,
}

impl MapSize {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}
