use crate::prelude::*;
use bevy::prelude::*;

/// Represents a tile.
#[derive(Clone, Component, Copy)]
pub struct Tile {
    pub kind: TileKind,
    pub actor: Option<Actor>,
}

/// Represent all kind of tiles.
#[derive(Clone, Copy, Default)]
pub enum TileKind {
    #[default]
    Grass,
    GrassWithFlower,
    GrassWithStone,
    LevelExit,
}

impl Default for Tile {
    fn default() -> Self {
        Self {
            kind: TileKind::default(),
            actor: None,
        }
    }
}

impl Tile {
    /// Creates a new `Tile` with a given `TileKind`. Other fields are set
    /// with default values.
    pub fn from_kind(kind: TileKind) -> Self {
        Self { kind, actor: None }
    }

    /// Returns whether or not a tile can be walked on by an actor.
    pub const fn is_walkable(self) -> bool {
        match self.kind {
            TileKind::Grass
            | TileKind::GrassWithFlower
            | TileKind::LevelExit => self.actor.is_none(),
            TileKind::GrassWithStone => false,
        }
    }
}
impl TileKind {
    /// Returns the sprite index for a given `TileType`. The index corresponds
    /// to the location in the tilesheet where the corresponding tile is.
    pub const fn to_sprite_idx(kind: Self) -> usize {
        match kind {
            Self::Grass => TILESET_TERRAIN_IDX_GRASS,
            Self::GrassWithFlower => TILESET_TERRAIN_IDX_GRASS_WITH_FLOWER,
            Self::GrassWithStone => TILESET_TERRAIN_IDX_GRASS_WITH_STONE,
            Self::LevelExit => TILESET_TERRAIN_IDX_SIGNPOST,
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
    /// The position on the map where the tile is.
    pub map_position: MapPosition,
}

impl TileBundle {
    /// Creates a new entity using the `TileBundle` struct components.
    pub fn new(
        map_position: MapPosition,
        tileset: &TilesetTerrain,
        tile: Tile,
    ) -> Self {
        let (sprite_x, sprite_y) = map_position.as_sprite_coordinates();
        Self {
            tile,
            map_position,
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
                    index: TileKind::to_sprite_idx(tile.kind),
                },
                ..Default::default()
            },
        }
    }
}
