use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
}

/// Creates an entity for the player.
pub fn initialize_player(
    commands: &mut Commands,
    map_position: MapPosition,
    tileset: &TilesetActor,
    current_map_number: usize,
) {
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
    commands.spawn((
        ActorBundle {
            actor: Actor,
            map_position: map_position,
            sprite: SpriteSheetBundle {
                atlas: TextureAtlas {
                    layout: tileset.0.clone(),
                    index: TILESET_ACTOR_IDX_PLAYER,
                },
                texture: tileset.1.clone(),
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_ACTOR,
                ),
                sprite: Sprite::default(),
                ..Default::default()
            },
            map_number: MapNumber(current_map_number),
        },
        PlayerBundle { player: Player },
    ));
}

/// Updates the player's sprite position based on its `MapPosition`.
pub fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut sprite_transform, position_player) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);
    sprite_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
