use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Rabbit;

impl Creature for Rabbit {
    fn new() -> Self {
        Self
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_RABBIT
    }
}
