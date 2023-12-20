use bevy::prelude::*;

use crate::consts::*;

#[derive(Component)]
pub struct UiTurnText;

pub fn spawn_turn_counter_text(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        UiTurnText,
        TextBundle::from_section(
            "Turn 12391-9231",
            TextStyle {
                font: asset_server.load("fonts/RubikDoodleShadow-Regular.ttf"),
                font_size: 100.0,
                color: UI_TEXT_TURN_COLOR,
            },
        ),
    ));
}
