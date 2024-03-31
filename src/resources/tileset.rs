use bevy::asset::LoadedFolder;
use bevy::prelude::*;

use crate::prelude::*;

#[derive(Default, Resource)]
pub struct TilesetFolder(pub Handle<LoadedFolder>);

#[derive(Default, Resource)]
pub struct TilesetActor(pub Handle<TextureAtlasLayout>);

#[derive(Default, Resource)]
pub struct TilesetTerrain(pub Handle<TextureAtlasLayout>);

pub fn initialize_tileset_actor_resource(
    handle: &UntypedHandle,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        TILESET_ACTOR_COLUMNS,
        TILESET_ACTOR_ROWS,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TilesetActor(atlas_handle));
}

pub fn initialize_tileset_terrain_resource(
    handle: &UntypedHandle,
    texture_atlases: &mut ResMut<Assets<TextureAtlasLayout>>,
    commands: &mut Commands,
) {
    let texture_atlas = TextureAtlasLayout::from_grid(
        Vec2::new(SPRITE_TILE_WIDTH, SPRITE_TILE_HEIGHT),
        TILESET_TERRAIN_COLUMNS,
        TILESET_TERRAIN_ROWS,
        None,
        None,
    );
    let atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(TilesetTerrain(atlas_handle));
}
