use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::scenes::AppState;
use crate::resources::audio::AudioAssets;

pub struct DesignerPlugin;

#[derive(Resource)]
pub struct DesignerEntity {
    pub entity: Entity,
}


impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Designer), start_designer_music);
    }
}

fn start_designer_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.main_menu.clone()).looped().with_volume(0.15);
}
