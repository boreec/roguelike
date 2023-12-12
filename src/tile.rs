use crate::consts::*;

#[derive(Clone)]
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
