use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Rabbit;

#[derive(Bundle)]
pub struct RabbitBundle {
    pub rabbit: Rabbit,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
    pub map_number: MapNumber,
}

pub fn initialize_rabbits(
    commands: &mut Commands,
    map: &Map,
    tileset: &TilesetActor,
    current_map_number: usize,
) {
    for _ in 0..3 {
        let map_position = map.generate_random_spawning_position().unwrap();
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        commands.spawn(RabbitBundle {
            rabbit: Rabbit,
            position: map_position,
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
        });
    }
}
