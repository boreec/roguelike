use crate::prelude::*;
use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_turn_counter_text);
    }
}

#[derive(Component)]
pub struct UiTurnText;

pub fn setup_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        UiTurnText,
        TextBundle::from_section(
            "Turn 0",
            TextStyle {
                font: asset_server.load("fonts/GABOED.ttf"),
                font_size: UI_TEXT_TURN_SIZE,
                color: UI_TEXT_TURN_COLOR,
            },
        ),
    ));
}

pub fn update_turn_counter_text(
    mut query: Query<&mut Text, With<UiTurnText>>,
    game_turn: Res<GameTurn>,
) {
    let mut text = query.single_mut();
    text.sections[0].value = format!("TURN #{}", game_turn.current);
}
