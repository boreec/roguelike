use crate::consts::*;
use crate::map::MapPosition;

use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component)]
pub enum TileType {
    Grass,
    GrassWithFlower,
}

impl TileType {
    pub fn to_sprite_idx(&self) -> usize {
        match self {
            TileType::Grass => SPRITE_IDX_GRASS,
            TileType::GrassWithFlower => SPRITE_IDX_GRASS_WITH_FLOWER,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    tile: Tile,
    r#type: TileType,
    position: MapPosition,
}
