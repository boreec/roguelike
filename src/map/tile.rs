use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Tile;

#[derive(Clone, Component, Copy)]
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

impl TileBundle {
    /// Creates a new entiy using the `TileBundle` struct components.
    pub fn new(
        map_position: MapPosition,
        map_number: usize,
        tileset: &TilesetTerrain,
        tile_type: TileType,
    ) -> Self {
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        Self {
            tile: Tile,
            r#type: tile_type,
            map_position,
            map_number: MapNumber(map_number),
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
                    index: TileType::to_sprite_idx(&tile_type),
                },
                ..Default::default()
            },
        }
    }
}
