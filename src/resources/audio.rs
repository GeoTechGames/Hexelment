use bevy_kira_audio::AudioSource;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/dance_with_danger_8bit.ogg")]
    pub main_menu: Handle<AudioSource>,
}
