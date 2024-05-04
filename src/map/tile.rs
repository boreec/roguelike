use crate::prelude::*;
use bevy::prelude::*;

/// Marker component to represent a tile.
#[derive(Component)]
pub struct Tile;

/// Enumeration to represent all types of tiles.
#[derive(Clone, Component, Copy)]
pub enum TileType {
    Grass,
    GrassWithFlower,
    GrassWithStone,
    LevelExit,
}

impl TileType {
    /// Returns the sprite index for a given `TileType`. The index corresponds
    /// to the location in the tilesheet where the corresponding tile is.
    pub const fn to_sprite_idx(tile_type: &Self) -> usize {
        match tile_type {
            Self::Grass => TILESET_TERRAIN_IDX_GRASS,
            Self::GrassWithFlower => TILESET_TERRAIN_IDX_GRASS_WITH_FLOWER,
            Self::GrassWithStone => TILESET_TERRAIN_IDX_GRASS_WITH_STONE,
            Self::LevelExit => TILESET_TERRAIN_IDX_SIGNPOST,
        }
    }

    /// Returns whether or not a tile can be walked on by an actor.
    pub const fn is_walkable(self) -> bool {
        match self {
            Self::Grass | Self::GrassWithFlower => true,
            Self::GrassWithStone => false,
            Self::LevelExit => true,
        }
    }
}

/// Bundle for creating an entity representing a tile.
#[derive(Bundle)]
pub struct TileBundle {
    /// Marker component for the tile.
    pub tile: Tile,
    /// Type of the tile.
    pub r#type: TileType,
    /// Sprite bundle for rendering the tile.
    pub sprite: SpriteSheetBundle,
    /// The number of the map where the tile is.
    pub map_number: MapNumber,
    /// The position on the map where the tile is.
    pub map_position: MapPosition,
}

impl TileBundle {
    /// Creates a new entity using the `TileBundle` struct components.
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
