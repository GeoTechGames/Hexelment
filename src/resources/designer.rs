use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct ImageAssets {
    // Tile highlighted
    #[asset(path = "designer/empty_tile.png")]
    pub highlighted: Handle<Image>,

    // Plot tiles
    #[asset(path = "designer/crops.png")]
    pub crops: Handle<Image>,
    #[asset(path = "designer/round_house.png")]
    pub round_house: Handle<Image>,
    #[asset(path = "designer/rectangular_house.png")]
    pub rectangular_house: Handle<Image>,
    #[asset(path = "designer/villa.png")]
    pub villa: Handle<Image>,

    // Terrain tiles
    #[asset(path = "designer/grassland.png")]
    pub grassland: Handle<Image>,
    #[asset(path = "designer/highland.png")]
    pub highland: Handle<Image>,
    #[asset(path = "designer/woodland.png")]
    pub woodland: Handle<Image>,
    #[asset(path = "designer/grassland_highland.png")]
    pub grassland_highland: Handle<Image>,
    #[asset(path = "designer/grassland_woodland.png")]
    pub grassland_woodland: Handle<Image>,
    #[asset(path = "designer/woodland_highland.png")]
    pub woodland_highland: Handle<Image>,
    #[asset(path = "designer/grassland_woodland_highland.png")]
    pub grassland_woodland_highland: Handle<Image>,
}
