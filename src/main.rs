mod actors;
mod camera;
mod cellular_automaton;
mod constants;
mod debug;
mod input;
mod map;
mod movement;
mod noise;
mod resources;
mod states;
mod ui;

mod prelude {
    pub use crate::actors::*;
    pub use crate::calculate_sprite_position;
    pub use crate::camera::*;
    pub use crate::cellular_automaton::*;
    pub use crate::constants::*;
    pub use crate::debug::*;
    pub use crate::input::*;
    pub use crate::map::*;
    pub use crate::movement::*;
    pub use crate::noise::*;
    pub use crate::resources::*;
    pub use crate::states::*;
    pub use crate::ui::*;
    pub use bevy::asset::LoadedFolder;
    pub use bevy::prelude::*;
    pub use rand::prelude::*;
}

use prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::ANTIQUE_WHITE))
        .insert_resource(CurrentTurnNumber::default())
        .insert_resource(CurrentMapNumber::default())
        .insert_resource(Msaa::Off)
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        title: WINDOW_TITLE.to_string(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
            ActorsPlugin,
            CameraPlugin,
            MapPlugin,
            ResourcesPlugin,
            DebugPlugin,
            UiPlugin,
        ))
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_systems(OnEnter(AppState::LoadingAssets), load_assets)
        .add_systems(
            Update,
            check_assets.run_if(in_state(AppState::LoadingAssets)),
        )
        .add_systems(
            Update,
            (check_player_input, check_exit_events, update_player_sprite)
                .run_if(in_state(GameState::PlayerTurn)),
        )
        .add_systems(OnEnter(GameState::EnemyTurn), increase_game_turn)
        .run();
}

/// Loads assets from the assets folder and creates the appropriate
/// resources.
fn load_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("asset loading...");
    commands.insert_resource(TilesetFolder(
        asset_server.load_folder("img/tileset"),
    ));
}

/// Checks if all assets are properly loaded. The application state is switched
/// only after everything is loaded.
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

/// Calculates where a sprite should be displayed in the window depending on
/// its map position.
pub fn calculate_sprite_position(map_position: &MapPosition) -> (f32, f32) {
    (
        map_position.x as f32 * SPRITE_TILE_WIDTH + SPRITE_TILE_WIDTH / 2.0,
        -1f32 * map_position.y as f32 * SPRITE_TILE_HEIGHT
            - SPRITE_TILE_HEIGHT / 2.0,
    )
}
