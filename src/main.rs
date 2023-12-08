use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, check_exit_events)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("img/tileset.png"),
        Vec2::new(192f32, 64f32),
        3,
        1,
        None,
        None,
    );

    commands.spawn(SpriteBundle {
        texture: texture_atlas.texture.clone(),
        ..Default::default()
    });
}

fn check_exit_events(
    input: Res<Input<KeyCode>>,
    mut exit_events: ResMut<Events<bevy::app::AppExit>>,
) {
    if input.just_pressed(KeyCode::Escape) {
        exit_events.send(bevy::app::AppExit);
    }
}
