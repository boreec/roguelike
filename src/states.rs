use bevy::prelude::*;

/// States used for the whole executable application. It comprises resources
/// loading, game switching, etc.
///
/// The lifecycle of the game is:
/// 1. `LoadingAssets` -> `InGame`
/// 2. `InGame` -> `Finished`
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    /// First state for the whole application, consisting of loading the
    /// assets and creating the game resources.
    #[default]
    LoadingAssets,
    /// Corresponds to the main game state.
    InGame,
    /// This state is used to exit the application cleanly, performing
    /// potential resources cleanup.
    Finished,
}

/// States used exclusively during the game. It involves not only the map and
/// actors creation, but also the main game turn between the player and the
/// enemies.
///
/// The lifecycle of the game is:
/// 1. `Unitialized` -> `InitializingMap`
/// 2. `InitializingMap` -> `InitializingActors`
/// 3. `InitializingActors` -> `PlayerTurn`
/// 4.
///   1. `PlayerTurn` -> `EnemyTurn`
///   2. `PlayerTurn` -> `CleanupActors`
/// 5.
///   1. `EnemyTurn` -> `PlayerTurn` (back to step 4.1)
///   2. `CleanupActors` -> `CleanupMap`
/// 6. `CleanupMap` -> `InitializingMap` (back to step 2)
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

/// States used for switching between release and debugging modes.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum ExecutionMode {
    /// Corresponds to the mode intended to be played.
    #[default]
    Release,
    /// Corresponds to the mode used by developper, containing extra bypass and
    /// information useful for developping and debugging the game.
    Debug,
}

impl ExecutionMode {
    /// Flips the current mode.
    /// `Debug` -> `Release`
    /// `Release` -> `Debug`
    pub fn flip(&mut self) {
        *self = match *self {
            Self::Release => Self::Debug,
            Self::Debug => Self::Release,
        }
    }
}
