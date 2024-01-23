use crate::prelude::*;
use bevy::prelude::*;

#[derive(Clone, Resource)]
pub enum GridState {
    On,
    Off,
}

impl GridState {
    pub fn flip(&mut self) {
        *self = match *self {
            Self::On => Self::Off,
            Self::Off => Self::On,
        }
    }
}

#[derive(Component)]
pub struct Grid;

pub fn spawn_grid_vertical_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.height as f32 * SPRITE_TILE_HEIGHT;
    for i in 0..=map.width {
        let position_anchor = MapPosition { x: i, y: 0 };
        let (line_x, _) = calculate_sprite_position(&position_anchor);
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(GRID_LINE_WIDTH, line_length)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    line_x - SPRITE_TILE_WIDTH / 2.0,
                    map.height as f32 * SPRITE_TILE_HEIGHT / -2.,
                    Z_INDEX_GRID,
                ),
                ..default()
            },
        ));
    }
}

pub fn spawn_grid_horizontal_lines(commands: &mut Commands, map: &Map) {
    let line_length = map.width as f32 * SPRITE_TILE_WIDTH;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0, y: j };
        let (_, line_y) = calculate_sprite_position(&position_anchor);
        commands.spawn((
            Grid,
            SpriteBundle {
                sprite: Sprite {
                    color: GRID_COLOR,
                    custom_size: Some(Vec2::new(line_length, GRID_LINE_WIDTH)),
                    ..default()
                },
                transform: Transform::from_xyz(
                    map.width as f32 * SPRITE_TILE_HEIGHT / 2.,
                    line_y + SPRITE_TILE_HEIGHT - SPRITE_TILE_HEIGHT / 2.,
                    Z_INDEX_GRID,
                ),
                ..default()
            },
        ));
    }
}

pub fn despawn_grid_lines(commands: &mut Commands, grid_entities: Vec<Entity>) {
    for entity in &grid_entities {
        commands.entity(*entity).despawn();
    }
}

pub fn display_grid(
    mut commands: Commands,
    query_map: Query<&Map>,
    query_grid_entities: Query<Entity, With<Grid>>,
    mut grid_state: ResMut<GridState>,
    input: Res<Input<KeyCode>>,
) {
    match grid_state.clone() {
        GridState::On => {
            despawn_grid_lines(
                &mut commands,
                query_grid_entities.iter().collect(),
            );
        }
        GridState::Off => {
            let map = query_map.single();
            spawn_grid_vertical_lines(&mut commands, map);
            spawn_grid_horizontal_lines(&mut commands, map);
        }
    }
    grid_state.flip();
}
