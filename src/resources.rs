use bevy::{asset::LoadedFolder, prelude::*};

use crate::GameState;

#[derive(Default, Resource)]
pub struct TilesetFolder(pub Handle<LoadedFolder>);

#[derive(Default, Resource)]
pub struct GameTurn {
    pub current: usize,
}

pub fn increase_game_turn(
    mut next_state: ResMut<NextState<GameState>>,
    mut game_turn: ResMut<GameTurn>,
) {
    game_turn.current += 1;
    next_state.set(GameState::PlayerTurn);
}
