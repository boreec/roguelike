use crate::map::MapPosition;

use bevy::prelude::Bundle;
use bevy::prelude::Component;
use bevy::prelude::SpriteSheetBundle;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub position: MapPosition,
    pub sprite: SpriteSheetBundle,
}
