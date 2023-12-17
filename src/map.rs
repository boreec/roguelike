use bevy::prelude::Component;

#[derive(Component)]
pub struct Map {
    pub width: usize,
    pub height: usize,
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
