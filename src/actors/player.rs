use crate::prelude::*;
use bevy::prelude::*;

/// Marker component to represent a `Player` entity.
#[derive(Component)]
pub struct Player;

impl Creature for Player {
    fn new() -> Self {
        Self
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_PLAYER
    }
}
