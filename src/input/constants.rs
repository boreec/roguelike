use bevy::prelude::*;

pub const KEY_PLAYER_SKIP_TURN: KeyCode = KeyCode::Space;
pub const KEYS_PLAYER_MOVE_LEFT: [KeyCode; 2] =
    [KeyCode::KeyA, KeyCode::ArrowLeft];
pub const KEYS_PLAYER_MOVE_RIGHT: [KeyCode; 2] =
    [KeyCode::KeyD, KeyCode::ArrowRight];
pub const KEYS_PLAYER_MOVE_UP: [KeyCode; 2] = [KeyCode::KeyW, KeyCode::ArrowUp];
pub const KEYS_PLAYER_MOVE_DOWN: [KeyCode; 2] =
    [KeyCode::KeyS, KeyCode::ArrowDown];

pub const KEY_APP_EXIT: KeyCode = KeyCode::Escape;
