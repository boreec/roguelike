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

/// Updates the player's sprite position based on its `MapPosition`.
pub fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut sprite_transform, position_player) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);
    sprite_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
