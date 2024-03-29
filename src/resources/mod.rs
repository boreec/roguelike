mod constants;
mod tileset;

pub use constants::*;
pub use tileset::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), initialize_resources);
    }
}

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
