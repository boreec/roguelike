use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Blob;

#[derive(Bundle)]
pub struct BlobBundle {
    pub blob: Blob,
}

impl BlobBundle {
    pub fn new() -> Self {
        Self { blob: Blob }
    }
}

impl Creature for BlobBundle {
    fn new_bundle() -> impl Bundle {
        BlobBundle::new()
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_BLOB
    }
}
