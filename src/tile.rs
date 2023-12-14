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
}

impl TileType {
    pub const fn to_sprite_idx(&self) -> usize {
        match self {
            Self::Grass => SPRITE_IDX_GRASS,
            Self::GrassWithFlower => SPRITE_IDX_GRASS_WITH_FLOWER,
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
