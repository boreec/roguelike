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

pub fn initialize_rabbits(
    commands: &mut Commands,
    rabbit_spawn_positions: &Vec<MapPosition>,
    tileset: &TilesetActor,
    current_map_number: usize,
) {
    for map_position in rabbit_spawn_positions {
        commands.spawn((
            ActorBundle::new(
                *map_position,
                current_map_number,
                tileset,
                TILESET_ACTOR_IDX_RABBIT,
            ),
            RabbitBundle::new(),
        ));
    }
}
