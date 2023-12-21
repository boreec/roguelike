use crate::prelude::*;
use bevy::prelude::*;

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
            Self::Grass => SPRITE_IDX_GRASS,
            Self::GrassWithFlower => SPRITE_IDX_GRASS_WITH_FLOWER,
            Self::GrassWithStone => SPRITE_IDX_GRASS_WITH_STONE,
        }
    }

    pub const fn is_walkable(self) -> bool {
        match self {
            Self::Grass => true,
            Self::GrassWithFlower => true,
            Self::GrassWithStone => false,
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
