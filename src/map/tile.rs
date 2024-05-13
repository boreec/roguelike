use crate::prelude::*;
use bevy::prelude::*;

/// Marker component to represent a tile.
#[derive(Component)]
pub struct Tile {
    pub kind: TileKind,
}

/// Enumeration to represent all types of tiles.
#[derive(Clone, Copy)]
pub enum TileKind {
    Grass,
    GrassWithFlower,
    GrassWithStone,
    LevelExit,
}

impl TileKind {
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
            Self::Grass | Self::GrassWithFlower | Self::LevelExit => true,
            Self::GrassWithStone => false,
        }
    }
}

/// Bundle for creating an entity representing a tile.
#[derive(Bundle)]
pub struct TileBundle {
    /// Marker component for the tile.
    pub tile: Tile,
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
        tile_type: TileKind,
    ) -> Self {
        let (sprite_x, sprite_y) = map_position.as_sprite_coordinates();
        Self {
            tile: Tile { kind: tile_type },
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
                    index: TileKind::to_sprite_idx(&tile_type),
                },
                ..Default::default()
            },
        }
    }
}
