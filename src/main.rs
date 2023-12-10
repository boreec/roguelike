use bevy::prelude::*;

const WINDOW_WITDH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 800.0;

const SPRITESHEET_COLS: usize = 3;
const SPRITESHEET_ROWS: usize = 1;
const SPRITE_TILE_WIDTH: f32 = 64.0;
const SPRITE_TILE_HEIGHT: f32 = 64.0;

const SPRITE_IDX_GRASS: usize = 0;
const SPRITE_IDX_GRASS_WITH_FLOWER: usize = 1;
const SPRITE_IDX_PLAYER: usize = 2;

const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 10;

#[derive(Clone)]
enum TileType {
    Grass,
}

#[derive(Component)]
struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    fn new(width: usize, height: usize) -> Self {
        return Map {
            tiles: vec![TileType::Grass; width * height],
        };
    }
}

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

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(100f32, 100f32, 0f32),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(SPRITE_IDX_PLAYER),
        texture_atlas: atlas_handle.clone(),
        ..Default::default()
    });

    let map = Map::new(MAP_WIDTH, MAP_HEIGHT);
    let top_left_x = WINDOW_WITDH / -2.0;
    let top_left_y = WINDOW_HEIGHT / 2.0;
    for (tile_i, _tile_type) in map.tiles.iter().enumerate() {
        let tile_x = (tile_i % MAP_WIDTH) as f32;
        let tile_y = (tile_i / MAP_WIDTH) as f32;
        commands.spawn(SpriteSheetBundle {
            transform: Transform::from_xyz(
                top_left_x
                    + tile_x * SPRITE_TILE_WIDTH
                    + SPRITE_TILE_WIDTH / 2.0,
                top_left_y - tile_y * SPRITE_TILE_HEIGHT,
                0f32,
            ),
            sprite: TextureAtlasSprite::new(SPRITE_IDX_GRASS),
            texture_atlas: atlas_handle.clone(),
            ..Default::default()
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
