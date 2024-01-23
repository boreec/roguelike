use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    LoadingAssets,
    InGame,
    Finished,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    Uninitialized,
    InitializingMap,
    InitializingPlayer,
    PlayerTurn,
    EnemyTurn,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ExecutionMode {
    #[default]
    Release,
    Debug,
}

impl ExecutionMode {
    pub fn flip(&mut self) {
        *self = match *self {
            Self::Release => Self::Debug,
            Self::Debug => Self::Release,
        }
    }
}
