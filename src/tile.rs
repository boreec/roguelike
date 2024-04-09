use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component)]
pub enum TileType {
    Grass,
    GrassWithFlower,
    GrassWithStone,
    LevelExit,
}

impl TileType {
    pub const fn to_sprite_idx(tile_type: &Self) -> usize {
        match tile_type {
            Self::Grass => TILESET_TERRAIN_IDX_GRASS,
            Self::GrassWithFlower => TILESET_TERRAIN_IDX_GRASS_WITH_FLOWER,
            Self::GrassWithStone => TILESET_TERRAIN_IDX_GRASS_WITH_STONE,
            Self::LevelExit => TILESET_TERRAIN_IDX_SIGNPOST,
        }
    }

    pub const fn is_walkable(&self) -> bool {
        match self {
            Self::Grass | Self::GrassWithFlower => true,
            Self::GrassWithStone => false,
            Self::LevelExit => true,
        }
    }
}

#[derive(Bundle)]
pub struct TileBundle {
    pub tile: Tile,
    pub r#type: TileType,
    pub sprite: SpriteSheetBundle,
    pub map_number: MapNumber,
    pub map_position: MapPosition,
}
