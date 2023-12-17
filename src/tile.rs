use crate::consts::*;
use crate::map::MapPosition;

use bevy::prelude::Bundle;
use bevy::prelude::Component;
use bevy::prelude::SpriteSheetBundle;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component)]
pub enum TileType {
    Grass,
    GrassWithFlower,
    GrassWithStone,
}

impl TileType {
    pub const fn to_sprite_idx(tile_type: &TileType) -> usize {
        match tile_type {
            TileType::Grass => SPRITE_IDX_GRASS,
            TileType::GrassWithFlower => SPRITE_IDX_GRASS_WITH_FLOWER,
            TileType::GrassWithStone => SPRITE_IDX_GRASS_WITH_STONE,
        }
    }

    pub const fn is_walkable(tile_type: &TileType) -> bool {
        match tile_type {
            TileType::Grass => true,
            TileType::GrassWithFlower => true,
            TileType::GrassWithStone => false,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub r#type: TileType,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}
