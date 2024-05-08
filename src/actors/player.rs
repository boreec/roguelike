use crate::prelude::*;
use bevy::prelude::*;

/// Marker component to represent a `Player` entity.
#[derive(Component)]
pub struct Player;

/// Bundle used for creating a `Player` entity.
#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
}

impl PlayerBundle {
    pub fn new() -> Self {
        Self { player: Player }
    }
}

impl Creature for PlayerBundle {
    fn new_bundle() -> impl Bundle {
        PlayerBundle::new()
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_PLAYER
    }
}
