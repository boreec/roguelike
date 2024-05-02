use bevy::prelude::Color;

/// The window's width in pixels.
pub const WINDOW_WIDTH: f32 = 600.0;

/// The window's height in pixels.
pub const WINDOW_HEIGHT: f32 = 800.0;

/// The window's title.
pub const WINDOW_TITLE: &str = "roguelike";

/// The width in pixels for sprites representing tiles.
pub const SPRITE_TILE_WIDTH: f32 = 64.0;

/// The height in pixels for sprites representing tiles.
pub const SPRITE_TILE_HEIGHT: f32 = 64.0;

pub const Z_INDEX_TILE: f32 = 0.0;

pub const UI_TEXT_TURN_COLOR: Color = Color::BLACK;
pub const UI_TEXT_TURN_SIZE: f32 = 20.0;

pub const PERLIN_NOISE_SCALE: f64 = 0.1;
