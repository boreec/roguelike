use bevy::prelude::*;

mod consts;
mod map;
mod player;
mod tile;

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
        .add_systems(Update, check_exit_events)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

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
    commands.spawn(PlayerBundle {
        player: Player,
        position: MapPosition::new(0, 0),
        sprite: SpriteSheetBundle {
            texture_atlas: atlas_handle.clone(),
            transform: Transform::from_xyz(0.0, 0.0, Z_INDEX_PLAYER),
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
    let top_left_x = WINDOW_WITDH / -2.0;
    let top_left_y = WINDOW_HEIGHT / 2.0;
    for (tile_i, tile_type) in map.tiles.iter().enumerate() {
        let tile_x = tile_i % MAP_WIDTH;
        let tile_y = tile_i / MAP_WIDTH;
        let sprite_x = top_left_x
            + tile_x as f32 * SPRITE_TILE_WIDTH
            + SPRITE_TILE_WIDTH / 2.0;
        let sprite_y = top_left_y
            - tile_y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0;
        commands.spawn(TileBundle {
            tile: Tile,
            r#type: TileType::Grass,
            position: MapPosition::new(tile_x, tile_y),
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

fn check_exit_events(
    input: Res<Input<KeyCode>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit);
    }
}
