use bevy::prelude::*;

mod camera;
mod consts;
mod map;
mod player;
mod tile;

use camera::MainCamera;
use consts::*;
use map::*;
use player::*;

use tile::Tile;
use tile::TileBundle;
use tile::TileType;

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
                check_player_movement,
                update_camera_position,
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

    let map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    spawn_tiles(&mut commands, &map, &atlas_handle);
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
fn spawn_tiles(
    commands: &mut Commands,
    map: &Map,
    atlas_handle: &Handle<TextureAtlas>,
) {
    for (tile_i, tile_type) in map.tiles.iter().enumerate() {
        let map_position = MapPosition {
            x: tile_i % map.width,
            y: tile_i / map.width,
        };
        let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
        commands.spawn(TileBundle {
            tile: Tile,
            r#type: TileType::Grass,
            position: map_position,
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_TILE,
                ),
                sprite: TextureAtlasSprite::new(tile_type.to_sprite_idx()),
                texture_atlas: atlas_handle.clone(),
                ..Default::default()
            },
        });
    }
}

fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    let top_left_x = WINDOW_WITDH / -2.0;
    let top_left_y = WINDOW_HEIGHT / 2.0;
    (
        top_left_x
            + map_position.x as f32 * SPRITE_TILE_WIDTH
            + SPRITE_TILE_WIDTH / 2.0,
        top_left_y
            - map_position.y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0,
    )
}

fn check_player_movement(
    mut query: Query<&mut MapPosition, With<Player>>,
    input: Res<Input<KeyCode>>,
) {
    for mut position in query.iter_mut() {
        if input.any_just_pressed([KeyCode::Right, KeyCode::D]) {
            position.x += 1;
        }

        if input.any_just_pressed([KeyCode::Left, KeyCode::A]) {
            position.x -= 1;
        }

        if input.any_just_pressed([KeyCode::Up, KeyCode::W]) {
            position.y -= 1;
        }

        if input.any_just_pressed([KeyCode::Down, KeyCode::S]) {
            position.y += 1;
        }
    }
}

fn check_exit_events(
    input: Res<Input<KeyCode>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit);
    }
}

fn update_camera_position(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
    mut query_main_camera: Query<
        (&mut Transform, &MainCamera),
        Without<Player>,
    >,
) {
    let (mut transform, map_position) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);

    let (mut camera_transform, camera) = query_main_camera.single_mut();
    camera_transform.translation =
        Vec3::new(sprite_x, sprite_y, Z_INDEX_PLAYER);
}
