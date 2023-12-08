use bevy::prelude::*;

const SPRITESHEET_COLS: usize = 3;
const SPRITESHEET_ROWS: usize = 1;
const SPRITE_TILE_WIDTH: f32 = 64.0;
const SPRITE_TILE_HEIGHT: f32 = 64.0;
const SPRITE_IDX_PLAYER: usize = 2;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.9)))
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, check_exit_events)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());

    let texture_atlas = TextureAtlas::from_grid(
        asset_server.load("img/tileset.png"),
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        SPRITESHEET_COLS,
        SPRITESHEET_ROWS,
        None,
        None,
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(SpriteSheetBundle {
        transform: Transform {
            translation: Vec3::new(100f32, 100f32, 0f32),
            ..Default::default()
        },
        sprite: TextureAtlasSprite::new(SPRITE_IDX_PLAYER),
        texture_atlas: atlas_handle,
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
