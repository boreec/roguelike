mod camera;
mod consts;
mod debug;
mod input;
mod map;
mod movement;
mod player;
mod resources;
mod states;
mod tile;
mod ui;

mod prelude {
    pub use crate::calculate_sprite_position;
    pub use crate::camera::*;
    pub use crate::consts::*;
    pub use crate::debug::*;
    pub use crate::input::*;
    pub use crate::map::*;
    pub use crate::movement::*;
    pub use crate::player::*;
    pub use crate::resources::*;
    pub use crate::states::*;
    pub use crate::tile::*;
    pub use crate::ui::*;
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(GridState::Off)
        .insert_resource(GameTurn::default())
        .add_plugins((
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
            CameraPlugin,
            DebugPlugin,
            UiPlugin,
        ))
        .add_state::<GameState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            check_player_input.run_if(in_state(GameState::PlayerTurn)),
        )
        .add_systems(OnEnter(GameState::EnemyTurn), increase_game_turn)
        .add_systems(Update, (check_exit_events, update_player_sprite))
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
    spawn_turn_counter_text(&mut commands, asset_server);
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

pub fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    (
        map_position.x as f32 * SPRITE_TILE_WIDTH + SPRITE_TILE_WIDTH / 2.0,
        -1f32 * map_position.y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0,
    )
}

fn update_player_sprite(
    mut query_player: Query<(&mut Transform, &MapPosition), With<Player>>,
) {
    let (mut sprite_transform, position_player) = query_player.single_mut();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);
    sprite_transform.translation =
        Vec3::new(sprite_x, sprite_y, Z_INDEX_PLAYER);
}
