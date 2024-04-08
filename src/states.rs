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
    /// Corresponds to the default state, before the game is running.
    #[default]
    Uninitialized,
    /// Corresponds to the map creation.
    InitializingMap,
    /// Corresponds to the creation of the map's actors.
    InitializingActors,
    /// Corresponds to the turn when the player can do a move or an action.
    PlayerTurn,
    /// Corresponds to the turn when the enemies can do a move or an action.
    EnemyTurn,
    /// Corresponds to the map cleanup (spawned entities removal).
    CleanupMap,
    /// Corresponds to the map's actors cleanup (spawned entities removal).
    CleanupActors,
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
