mod constants;

use crate::prelude::*;
use constants::*;

use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_main_camera)
            .add_systems(
                Update,
                (check_camera_zoom, update_camera_position)
                    .run_if(in_state(GameState::PlayerTurn)),
            );
    }
}

/// Represents the camera displaying the `Player`, the `Map`, etc.
#[derive(Component)]
pub struct MainCamera;

/// Creates an entity for the `MainCamera`.
fn setup_main_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

/// Updates the camera zoom depending on the mouse wheel input.
pub fn check_camera_zoom(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query_main_camera: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    let mut projection = query_main_camera.single_mut();
    let mut log_scale = projection.scale.ln();
    for ev in scroll_evr.read() {
        if ev.unit != MouseScrollUnit::Line {
            continue;
        }
        if ev.y > 0f32 && projection.scale > CAMERA_ZOOM_IN_MAX {
            log_scale -= CAMERA_ZOOM_INCREMENT;
        } else if ev.y < 0f32 && projection.scale < CAMERA_ZOOM_OUT_MAX {
            log_scale += CAMERA_ZOOM_INCREMENT;
        }
    }
    projection.scale = log_scale.exp();
}

/// Sets the camera position centered on the player.
pub fn update_camera_position(
    query_actors: Query<(&MapPosition, &MapNumber, &ActorType)>,
    mut query_main_camera: Query<&mut Transform, With<MainCamera>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    let (position_player, _, _) = query_actors
        .iter()
        .filter(|(_, m_n, a)| {
            m_n.0 == current_map_number.0 && **a == ActorType::Player
        })
        .last()
        .expect("no player found");

    let (sprite_x, sprite_y) = position_player.as_sprite_coordinates();
    let mut camera_transform = query_main_camera.single_mut();
    camera_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
