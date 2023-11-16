use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

use crate::config::TILEMAP;
use crate::resources;
use crate::scenes::AppState;


pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TilemapPlugin)
            .add_systems(OnEnter(AppState::Simulation), generate);
    }
}



fn generate(mut commands: Commands, terrain_assets: Res<resources::terrain::ImageAssets>) {

    let map_size = TilemapSize {x: 48, y: 48};
    let chunk_size = TilemapSize {x: 16, y: 16};

    let mut tile_storage = TileStorage::empty(map_size);
    let tilemap_entity = commands.spawn_empty().id();
    let tilemap_id = TilemapId(tilemap_entity);

    for chunk_y in 0..(map_size.y / chunk_size.y) {
        for chunk_x in 0..(map_size.x / chunk_size.x) {
            fill_tilemap_rect(
                TileTextureIndex(*TILEMAP.get(&"grassland").unwrap()),
                TilePos { x: chunk_x * 16, y: chunk_y * 16 },
                chunk_size,
                tilemap_id,
                &mut commands,
                &mut tile_storage,
            );
        }
    }

    let tile_size = TilemapTileSize { x: 28.0, y: 32.0 };
    let grid_size = TilemapGridSize { x: 28.0, y: 32.0 };
    let map_type = TilemapType::Hexagon(HexCoordSystem::RowEven);

    commands.entity(tilemap_entity).insert(TilemapBundle {
        grid_size,
        size: map_size,
        storage: tile_storage,
        texture: TilemapTexture::Single(terrain_assets.tiles.clone()),
        tile_size,
        map_type,
        transform: get_tilemap_center_transform(&map_size, &grid_size, &map_type, 0.0),
        ..Default::default()
    });
}
