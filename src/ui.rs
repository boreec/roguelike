use crate::prelude::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_ui)
            .add_systems(
                Update,
                update_turn_counter_text.run_if(in_state(AppState::InGame)),
            );
    }
}

#[derive(Component)]
pub struct UiCurrentTurnText;

pub fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_turn_number: Res<CurrentTurnNumber>,
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
}

pub fn update_turn_counter_text(
    mut query: Query<&mut Text, With<UiCurrentTurnText>>,
    game_turn: Res<CurrentTurnNumber>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("TURN #{}", game_turn.0);
}
