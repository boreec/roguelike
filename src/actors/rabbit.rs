use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Rabbit;

#[derive(Bundle)]
pub struct RabbitBundle {
    pub rabbit: Rabbit,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}

pub fn initialize_rabbits(
    commands: &mut Commands,
    map: &Map,
    tileset: &TilesetActor,
) {
    for _ in 0..3 {
        let map_position = map.generate_random_spawning_position();
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        commands.spawn(RabbitBundle {
            rabbit: Rabbit,
            position: map_position,
            sprite: SpriteSheetBundle {
                texture_atlas: tileset.0.clone(),
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_ACTOR,
                ),
                sprite: TextureAtlasSprite::new(TILESET_ACTOR_IDX_RABBIT),
                ..Default::default()
            },
        });
    }
}
