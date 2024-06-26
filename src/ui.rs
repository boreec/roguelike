use crate::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(
                OnEnter(GameState::PlayerTurn),
                update_ui_current_turn_text.run_if(in_state(AppState::InGame)),
            )
            .add_systems(
                OnEnter(GameState::InitializingMap),
                update_ui_current_map_text.run_if(in_state(AppState::InGame)),
            );
    }
}

/// Marker component to represent the ui element to display the current turn
/// number.
#[derive(Component)]
pub struct UiCurrentTurnText;

/// Marker component to represent the ui element to display the current map
/// number.
#[derive(Component)]
pub struct UiCurrentMapText;

/// Creates components for the ui elements.
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
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.0),
            right: Val::Px(0.0),
            ..default()
        }),
    ));
}

/// Updates the ui element which represents the current turn.
pub fn update_ui_current_turn_text(
    mut q_text: Query<&mut Text, With<UiCurrentTurnText>>,
    current_turn_number: Res<CurrentTurnNumber>,
) {
    let mut text = q_text.single_mut();
    text.sections[0].value = format!("Turn {}", current_turn_number.0);
}

/// Updates the ui element which represents the current map.
pub fn update_ui_current_map_text(
    mut q_text: Query<&mut Text, With<UiCurrentMapText>>,
    current_map_number: Res<CurrentMapNumber>,
) {
    let mut text = q_text.single_mut();
    text.sections[0].value = format!("Map {}", current_map_number.0);
}
