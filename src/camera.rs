use crate::prelude::*;
use bevy::input::mouse::MouseWheel;
use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (check_camera_zoom, update_camera_position));
    }
}

#[derive(Component)]
pub struct MainCamera;

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

pub fn update_camera_position(
    query_player: Query<&MapPosition, With<Player>>,
    mut query_main_camera: Query<
        &mut Transform,
        (With<MainCamera>, Without<Player>),
    >,
) {
    let position_player = query_player.single();
    let (sprite_x, sprite_y) = calculate_sprite_position(position_player);

    let mut camera_transform = query_main_camera.single_mut();
    camera_transform.translation = Vec3::new(sprite_x, sprite_y, Z_INDEX_ACTOR);
}
