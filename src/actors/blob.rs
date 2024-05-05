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

pub fn initialize_blobs(
    commands: &mut Commands,
    spawn_positions: &[MapPosition],
    tileset: &TilesetActor,
    current_map_number: usize,
) {
    for map_position in spawn_positions {
        commands.spawn((
            ActorBundle::new(
                *map_position,
                current_map_number,
                tileset,
                TILESET_ACTOR_IDX_BLOB,
            ),
            BlobBundle::new(),
        ));
    }
}
