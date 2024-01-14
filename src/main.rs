mod camera;
mod cellular_automaton;
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
    pub use crate::cellular_automaton::*;
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
    pub use bevy::asset::LoadedFolder;
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(GridState::Off)
        .insert_resource(GameTurn::default())
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WITDH, WINDOW_HEIGHT).into(),
                        title: WINDOW_TITLE.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            CameraPlugin,
            DebugPlugin,
            UiPlugin,
        ))
        .add_state::<AppState>()
        .add_state::<GameState>()
        .add_systems(OnEnter(AppState::LoadingAssets), load_assets)
        .add_systems(
            Update,
            check_assets.run_if(in_state(AppState::LoadingAssets)),
        )
        .add_systems(
            OnEnter(AppState::InGame),
            (initialize_resources, setup_game),
        )
        .add_systems(OnEnter(GameState::InitializingMap), initialize_map)
        .add_systems(OnEnter(GameState::InitializingPlayer), initialize_player)
        .add_systems(
            Update,
            (check_player_input, check_exit_events, update_player_sprite)
                .run_if(in_state(GameState::PlayerTurn)),
        )
        .add_systems(OnEnter(GameState::EnemyTurn), increase_game_turn)
        .run();
}

fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("asset loading...");
    commands.insert_resource(TilesetFolder(asset_server.load_folder("img")));
}

fn check_assets(
    mut app_next_state: ResMut<NextState<AppState>>,
    mut events: EventReader<AssetEvent<LoadedFolder>>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id: _ } = event {
            println!("asset loaded!");
            app_next_state.set(AppState::InGame);
        }
    }
}

fn initialize_resources(
    mut commands: Commands,
    tileset_folder: Res<TilesetFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_next_state: ResMut<NextState<GameState>>,
) {
    let folder = loaded_folders.get(&tileset_folder.0).unwrap();
    let texture_atlas = TextureAtlas::from_grid(
        folder.handles[0].clone().typed::<Image>(),
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );

    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TilesetMain(atlas_handle));

    game_next_state.set(GameState::InitializingMap);
}

fn setup_game(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn initialize_player(
    mut commands: Commands,
    mut game_next_state: ResMut<NextState<GameState>>,
    query_map: Query<&Map>,
    tileset: Res<TilesetMain>,
) {
    let map = query_map.single();
    let map_position = map.generate_random_spawning_position();
    let (sprite_x, sprite_y) = calculate_sprite_position(&map_position);
    commands.spawn(PlayerBundle {
        player: Player,
        position: map_position,
        sprite: SpriteSheetBundle {
            texture_atlas: tileset.0.clone(),
            transform: Transform::from_xyz(sprite_x, sprite_y, Z_INDEX_ACTOR),
            sprite: TextureAtlasSprite::new(SPRITE_IDX_PLAYER),
            ..Default::default()
        },
    });
    game_next_state.set(GameState::PlayerTurn);
}

fn initialize_map(
    mut commands: Commands,
    mut game_next_state: ResMut<NextState<GameState>>,
    tileset: Res<TilesetMain>,
) {
    let mut ca = CellularAutomaton::new(MAP_WIDTH, MAP_HEIGHT, 0.5);
    for _ in 0..50 {
        ca.transition();
    }
    ca.smooth();
    let m = Map::from(ca);

    for (i, tile) in m.tiles.iter().enumerate() {
        let tile_position = MapPosition {
            x: i % m.width,
            y: i / m.width,
        };
        let (sprite_x, sprite_y) = calculate_sprite_position(&tile_position);
        commands.spawn(TileBundle {
            tile: Tile,
            r#type: tile.clone(),
            position: tile_position,
            sprite: SpriteSheetBundle {
                transform: Transform::from_xyz(
                    sprite_x,
                    sprite_y,
                    Z_INDEX_TILE,
                ),
                sprite: TextureAtlasSprite::new(TileType::to_sprite_idx(tile)),
                texture_atlas: tileset.0.clone(),
                ..Default::default()
            },
        });
    }

    commands.spawn(m);

    game_next_state.set(GameState::InitializingPlayer);
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
    sprite_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
