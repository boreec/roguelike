mod constants;

use crate::prelude::*;
pub use constants::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_main_camera)
            .add_systems(
                Update,
                update_camera_position.run_if(in_state(GameState::PlayerTurn)),
            );
    }
}

/// Represents the camera displaying the `Player`, the `Map`, etc.
#[derive(Component)]
pub struct MainCamera;

/// Represents an entity that is on the screen and displayable by the camera.
#[derive(Component)]
pub struct OnDisplay;

/// Creates an entity for the `MainCamera`.
fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Sets the camera position centered on the player.
pub fn update_camera_position(
    query_actors: Query<(&MapPosition, &Actor), With<OnDisplay>>,
    mut query_main_camera: Query<&mut Transform, With<MainCamera>>,
) {
    let (pos_player, _) = query_actors
        .iter()
        .filter(|(_, a)| a.is_player())
        .last()
        .expect("no player found");

    let (x, y) = pos_player.as_sprite_coordinates();
    let mut camera_transform = query_main_camera.single_mut();
    camera_transform.translation = Vec3::new(x, y, Z_INDEX_ACTOR);
}
