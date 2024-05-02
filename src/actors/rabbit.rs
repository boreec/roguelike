use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Rabbit;

#[derive(Bundle)]
pub struct RabbitBundle {
    pub rabbit: Rabbit,
}

pub fn initialize_rabbits(
    commands: &mut Commands,
    rabbit_spawn_positions: &Vec<MapPosition>,
    tileset: &TilesetActor,
    current_map_number: usize,
) {
    for map_position in rabbit_spawn_positions {
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        commands.spawn((
            ActorBundle {
                actor: Actor,
                position: *map_position,
                sprite: SpriteSheetBundle {
                    atlas: TextureAtlas {
                        layout: tileset.0.clone(),
                        index: TILESET_ACTOR_IDX_RABBIT,
                    },
                    transform: Transform::from_xyz(
                        sprite_x,
                        sprite_y,
                        Z_INDEX_ACTOR,
                    ),
                    texture: tileset.1.clone(),
                    sprite: Sprite::default(),
                    ..Default::default()
                },
                map_number: MapNumber(current_map_number),
            },
            RabbitBundle { rabbit: Rabbit },
        ));
    }
}
