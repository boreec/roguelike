mod actors;
mod camera;
mod constants;
mod debug;
mod input;
mod map;
mod resources;
mod states;
mod ui;

mod prelude {
    pub use crate::actors::*;
    pub use crate::camera::*;
    pub use crate::constants::*;
    pub use crate::debug::*;
    pub use crate::input::*;
    pub use crate::map::*;
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
        .insert_resource(ClearColor(Color::BLACK))
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
            InputPlugin,
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
