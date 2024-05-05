mod tileset;

pub use tileset::*;

use crate::prelude::*;
use bevy::prelude::*;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentTurnNumber::default())
            .insert_resource(CurrentMapNumber::default())
            .add_systems(OnEnter(AppState::InGame), initialize_resources);
    }
}

/// Represents the current map number. The map number is increased every time
/// the player exits to another map.
#[derive(Default, Resource)]
pub struct CurrentMapNumber(pub usize);

/// Represents the current game turn. A turn is passed each time all actors
/// have performed an action or a move.
#[derive(Default, Resource)]
pub struct CurrentTurnNumber(pub usize);

/// Increases the `CurrentTurnNumber` value by 1.
pub fn increase_game_turn(
    mut next_state: ResMut<NextState<GameState>>,
    mut game_turn: ResMut<CurrentTurnNumber>,
) {
    game_turn.0 += 1;
    next_state.set(GameState::PlayerTurn);
}

/// Initializes image resources.
fn initialize_resources(
    mut commands: Commands,
    tileset_folder: Res<TilesetFolder>,
    loaded_folders: Res<Assets<LoadedFolder>>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
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
