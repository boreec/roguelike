use bevy::prelude::*;

use crate::GameState;

#[derive(Resource)]
pub struct GameTurn {
    pub current: usize,
}

impl Default for GameTurn {
    fn default() -> Self {
        Self { current: 0 }
    }
}

pub fn increase_game_turn(
    mut next_state: ResMut<NextState<GameState>>,
    mut game_turn: ResMut<GameTurn>,
) {
    game_turn.current += 1;
    next_state.set(GameState::PlayerTurn);
}
