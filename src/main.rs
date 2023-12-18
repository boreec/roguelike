mod camera;
mod consts;
mod input;
mod map;
mod movement;
mod player;
mod tile;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

use crate::camera::*;
use crate::consts::*;
use crate::input::*;
use crate::map::*;
use crate::player::*;
use crate::tile::*;

use rand::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WITDH, WINDOW_HEIGHT).into(),
                        title: "Havoc Resurgence".to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                check_exit_events,
                check_player_input,
                check_camera_zoom,
                update_camera_position,
                update_player_sprite,
                display_grid,
            ),
        )
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("img/tileset.png"),
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    spawn_map(&mut commands, &atlas_handle);
    spawn_player(&mut commands, &atlas_handle);
}

fn spawn_player(commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>) {
    let map_position = MapPosition::new(0, 0);
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
    commands.spawn(PlayerBundle {
        player: Player,
        position: map_position,
        sprite: SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(sprite_x, sprite_y, Z_INDEX_PLAYER),
            sprite: TextureAtlasSprite::new(SPRITE_IDX_PLAYER),
            ..Default::default()
        },
    });
}

fn spawn_map(commands: &mut Commands, atlas_handle: &Handle<TextureAtlas>) {
    let mut tiles = vec![];
    for i in 0..(MAP_WIDTH * MAP_HEIGHT) {
        let tile_position = MapPosition {
            x: i % MAP_WIDTH,
            y: i / MAP_WIDTH,
        };
        let (sprite_x, sprite_y) = calculate_sprite_position(&tile_position);
        let tile_type = {
            let mut rng = thread_rng();
            let throw = rng.gen_range(0..100);
            if throw < 25 {
                TileType::GrassWithFlower
            } else if throw < 50 {
                TileType::GrassWithStone
            } else {
                TileType::Grass
            }
        };
        tiles.push(tile_type.clone());
        commands.spawn(TileBundle {
            tile: Tile,
            r#type: tile_type.clone(),
            position: tile_position,
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_TILE,
                ),
                sprite: TextureAtlasSprite::new(TileType::to_sprite_idx(
                    &tile_type,
                )),
                texture_atlas: atlas_handle.clone(),
                ..Default::default()
            },
        });
    }

    commands.spawn(Map {
        width: MAP_WIDTH,
        height: MAP_HEIGHT,
        tiles,
    });
}

fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    (
        map_position.x as f32 * SPRITE_TILE_WIDTH + SPRITE_TILE_WIDTH / 2.0,
        -1f32 * map_position.y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0,
    )
}

pub fn check_camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query_main_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut projection = query_main_camera.single_mut();
    let mut log_scale = projection.scale.ln();
    for ev in scroll_evr.read() {
        if ev.unit != MouseScrollUnit::Line {
            continue;
        }
        if ev.y > 0f32 && projection.scale > CAMERA_ZOOM_IN_MAX {
            log_scale -= CAMERA_ZOOM_INCREMENT;
        } else if ev.y < 0f32 && projection.scale < CAMERA_ZOOM_OUT_MAX {
            log_scale += CAMERA_ZOOM_INCREMENT;
        }
    }
    projection.scale = log_scale.exp();
}

pub fn display_grid(mut commands: Commands, query_map: Query<&Map>) {
    let map = query_map.single();
    let mut line_length = map.height as f32 * SPRITE_TILE_HEIGHT;
    for i in 0..=map.width {
        let position_anchor = MapPosition { x: i, y: 0 };
        let (line_x, _) = calculate_sprite_position(&position_anchor);
        commands.spawn(SpriteBundle {
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
        });
    }

    line_length = map.width as f32 * SPRITE_TILE_WIDTH;
    for j in 0..=map.height {
        let position_anchor = MapPosition { x: 0, y: j };
        let (_, line_y) = calculate_sprite_position(&position_anchor);
        commands.spawn(SpriteBundle {
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
        });
    }
}

fn update_camera_position(
    query_player: Query<&MapPosition, With<Player>>,
    mut query_main_camera: Query<
        &mut Transform,
        (With<MainCamera>, Without<Player>),
    >,
) {
    let position_player = query_player.single();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);

    let mut camera_transform = query_main_camera.single_mut();
    camera_transform.translation =
        Vec3::new(sprite_x, sprite_y, Z_INDEX_PLAYER);
}

fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut sprite_transform, position_player) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);
    sprite_transform.translation =
        Vec3::new(sprite_x, sprite_y, Z_INDEX_PLAYER);
}
