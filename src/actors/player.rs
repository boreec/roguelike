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
    commands.spawn((
        ActorBundle::new(
            map_position,
            current_map_number,
            tileset,
            TILESET_ACTOR_IDX_PLAYER,
        ),
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
