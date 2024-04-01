use crate::prelude::*;
use bevy::prelude::*;

pub const DEBUG_MODE_KEY: KeyCode = KeyCode::KeyG;

pub const GRID_COLOR: Color = Color::BLACK;
pub const GRID_LINE_WIDTH: f32 = 2.0;

pub const TILE_COORDINATE_LABEL_FONT_COLOR: Color = Color::BLACK;
pub const TILE_COORDINATE_LABEL_FONT_SIZE: f32 = SPRITE_TILE_WIDTH * 0.25;

pub const Z_INDEX_GRID_LINES: f32 = 0.5;
pub const Z_INDEX_TILE_COORDINATES: f32 = 0.5;
