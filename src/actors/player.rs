use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}

pub fn initialize_player(
    mut commands: Commands,
    mut game_next_state: ResMut<NextState<GameState>>,
    query_map: Query<&Map>,
    tileset: Res<TilesetMain>,
) {
    let map = query_map.single();
    let map_position = map.generate_random_spawning_position();
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
    commands.spawn(PlayerBundle {
        player: Player,
        position: map_position,
        sprite: SpriteSheetBundle {
            texture_atlas: tileset.0.clone(),
            transform: Transform::from_xyz(sprite_x, sprite_y, Z_INDEX_ACTOR),
            sprite: TextureAtlasSprite::new(SPRITE_IDX_PLAYER),
            ..Default::default()
        },
    });
    game_next_state.set(GameState::PlayerTurn);
}

pub fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut sprite_transform, position_player) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);
    sprite_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
