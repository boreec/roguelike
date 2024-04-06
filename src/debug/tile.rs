use crate::debug::constants::*;
use crate::prelude::*;
use bevy::prelude::*;

#[derive(Component)]
pub struct TileCoordinateLabel;

pub fn spawn_tile_coordinate_labels(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    map: &Map,
) {
    for i in 0..map.height {
        for j in 0..map.width {
            let tile_position = MapPosition { x: j, y: i };
            let (text_x, text_y) = calculate_sprite_position(&tile_position);
            commands.spawn((
                TileCoordinateLabel,
                Text2dBundle {
                    text: Text::from_section(
                        format!("({j},{i})"),
                        TextStyle {
                            font: asset_server.load("fonts/GABOED.ttf"),
                            font_size: TILE_COORDINATE_LABEL_FONT_SIZE,
                            color: TILE_COORDINATE_LABEL_FONT_COLOR,
                        },
                    ),
                    transform: Transform::from_xyz(
                        text_x,
                        text_y,
                        Z_INDEX_TILE_COORDINATES,
                    ),
                    ..Default::default()
                },
            ));
        }
    }
}

pub fn despawn_tile_coordinate_labels(
    commands: &mut Commands,
    label_entities: Vec<Entity>,
) {
    for entity in &label_entities {
        commands.entity(*entity).despawn();
    }
}

pub fn show_tile_coordinate_labels(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query_map: Query<&Map>,
) {
    let map = query_map.single();
    spawn_tile_coordinate_labels(&mut commands, asset_server, map);
}

pub fn hide_tile_coordinate_labels(
    mut commands: Commands,
    query_label_entities: Query<Entity, With<TileCoordinateLabel>>,
) {
    despawn_tile_coordinate_labels(
        &mut commands,
        query_label_entities.iter().collect(),
    );
}
