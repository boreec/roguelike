use crate::prelude::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(
                Update,
                update_ui_current_turn_text.run_if(in_state(AppState::InGame)),
            );
    }
}

/// Marker component to represent the ui element to display the current turn.
#[derive(Component)]
pub struct UiCurrentTurnText;

#[derive(Component)]
pub struct UiCurrentMapText;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_turn_number: Res<CurrentTurnNumber>,
    current_map_number: Res<CurrentMapNumber>,
) {
    commands.spawn((
        UiCurrentTurnText,
        TextBundle::from_section(
            format!("Turn {}", current_turn_number.0),
            TextStyle {
                font: asset_server.load("fonts/GABOED.ttf"),
                font_size: UI_TEXT_TURN_SIZE,
                color: UI_TEXT_TURN_COLOR,
            },
        ),
    ));

    commands.spawn((
        UiCurrentMapText,
        TextBundle::from_section(
            format!("Map {}", current_map_number.0),
            TextStyle {
                font: asset_server.load("fonts/GABOED.ttf"),
                font_size: UI_TEXT_TURN_SIZE,
                color: UI_TEXT_TURN_COLOR,
            },
        ),
    ));
}

pub fn update_ui_current_turn_text(
    mut query: Query<&mut Text, With<UiCurrentTurnText>>,
    current_turn_number: Res<CurrentTurnNumber>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("TURN #{}", current_turn_number.0);
}
