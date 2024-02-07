mod constants;

pub use constants::*;

use crate::prelude::*;
use bevy::asset::LoadedFolder;
use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), initialize_resources);
    }
}

#[derive(Default, Resource)]
pub struct TilesetFolder(pub Handle<LoadedFolder>);

#[derive(Default, Resource)]
pub struct TilesetActor(pub Handle<TextureAtlas>);

#[derive(Default, Resource)]
pub struct TilesetTerrain(pub Handle<TextureAtlas>);

#[derive(Default, Resource)]
pub struct GameTurn {
    pub current: usize,
}

pub fn increase_game_turn(
    mut next_state: ResMut<NextState<GameState>>,
    mut game_turn: ResMut<GameTurn>,
) {
    game_turn.current += 1;
    next_state.set(GameState::PlayerTurn);
}

fn initialize_resources(
    mut commands: Commands,
    tileset_folder: Res<TilesetFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut game_next_state: ResMut<NextState<GameState>>,
) {
    let folder = loaded_folders.get(&tileset_folder.0).unwrap();

    for handle in &folder.handles {
        if let Some(path) = handle.path() {
            if let Some(stem) = path.path().file_stem() {
                if let Some(stem_str) = stem.to_str() {
                    match stem_str {
                        "actor" => {
                            initialize_tileset_actor_resource(
                                handle,
                                &mut texture_atlases,
                                &mut commands,
                            );
                        }
                        "terrain" => {
                            initialize_tileset_terrain_resource(
                                handle,
                                &mut texture_atlases,
                                &mut commands,
                            );
                        }
                        _ => {
                            panic!("tileset unused");
                        }
                    }
                }
            }
        }
    }

    game_next_state.set(GameState::InitializingMap);
}

fn initialize_tileset_actor_resource(
    handle: &UntypedHandle,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        handle.clone().typed::<Image>(),
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        TILESET_ACTOR_COLUMNS,
        TILESET_ACTOR_ROWS,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TilesetActor(atlas_handle));
}

fn initialize_tileset_terrain_resource(
    handle: &UntypedHandle,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlas::from_grid(
        handle.clone().typed::<Image>(),
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        TILESET_TERRAIN_COLUMNS,
        TILESET_TERRAIN_ROWS,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TilesetTerrain(atlas_handle));
}
