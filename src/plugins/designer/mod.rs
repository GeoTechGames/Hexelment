use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

use crate::scenes::AppState;
use crate::resources::audio::AudioAssets;


mod init;
mod interact;

pub struct DesignerPlugin;

#[derive(Resource)]
pub struct DesignerEntity {
    pub entity: Entity,
}

#[derive(Component)]
pub struct TileButton;


impl Plugin for DesignerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Designer), start_designer_music);
        app.add_systems(OnEnter(AppState::Designer), init::init_designer);
        app.add_systems(OnExit(AppState::Designer), cleanup);
        app.add_systems(Update, interact::tile_button_handle_system.run_if(in_state(AppState::Designer)));
    }
}

fn start_designer_music(audio_assets: Res<AudioAssets>, audio: Res<Audio>) {
    audio.stop();
    audio.play(audio_assets.main_menu.clone()).looped().with_volume(0.00);
}

fn cleanup(mut commands: Commands, entity_data: Res<DesignerEntity>) {
    commands
        .entity(entity_data.entity)
        .despawn_recursive();
}
