use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Rabbit;

#[derive(Bundle)]
pub struct RabbitBundle {
    pub rabbit: Rabbit,
}

impl RabbitBundle {
    pub fn new() -> Self {
        Self { rabbit: Rabbit }
    }
}

impl Creature for RabbitBundle {
    fn new_bundle() -> impl Bundle {
        RabbitBundle::new()
    }

    fn get_tileset_index() -> usize {
        TILESET_ACTOR_IDX_RABBIT
    }
}
