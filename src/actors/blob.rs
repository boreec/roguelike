use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Blob;

impl Creature for Blob {
    fn new() -> Self {
        Self
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_BLOB
    }
}
