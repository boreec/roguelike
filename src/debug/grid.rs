use crate::debug::constants::*;
use crate::prelude::*;
use bevy::prelude::*;

/// Marker component for the debug grid.
#[derive(Component)]
pub struct DebugGrid;

fn spawn_grid_vertical_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.height as f32 * SPRITE_TILE_HEIGHT;
    for i in 0..=map.width {
        let position_anchor = MapPosition { x: i, y: 0 };
        let (line_x, _) = position_anchor.as_sprite_coordinates();
        commands.spawn((
            DebugGrid,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(GRID_LINE_WIDTH, line_length)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    line_x - SPRITE_TILE_WIDTH / 2.0,
                    map.height as f32 * SPRITE_TILE_HEIGHT / -2.,
                    Z_INDEX_GRID_LINES,
                ),
                ..default()
            },
        ));
    }
}

fn spawn_grid_horizontal_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.width as f32 * SPRITE_TILE_WIDTH;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0, y: j };
        let (_, line_y) = position_anchor.as_sprite_coordinates();
        commands.spawn((
            DebugGrid,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(line_length, GRID_LINE_WIDTH)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    map.width as f32 * SPRITE_TILE_HEIGHT / 2.,
                    line_y + SPRITE_TILE_HEIGHT - SPRITE_TILE_HEIGHT / 2.,
                    Z_INDEX_GRID_LINES,
                ),
                ..default()
            },
        ));
    }
}

fn despawn_grid_lines(commands: &mut Commands, grid_entities: Vec<Entity>) {
    for entity in &grid_entities {
        commands.entity(*entity).despawn();
    }
}

pub fn show_grid(mut commands: Commands, query_map: Query<&Map>) {
    let map = query_map.single();
    spawn_grid_vertical_lines(&mut commands, map);
    spawn_grid_horizontal_lines(&mut commands, map);
}

pub fn hide_grid(
    mut commands: Commands,
    query_grid_entities: Query<Entity, With<DebugGrid>>,
) {
    despawn_grid_lines(&mut commands, query_grid_entities.iter().collect());
}
