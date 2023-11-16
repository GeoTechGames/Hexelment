#![allow(clippy::too_many_arguments, clippy::type_complexity, clippy::similar_names)]
#[macro_use]
extern crate lazy_static;

// add AI (https://github.com/zkat/big-brain)

// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy::{prelude::*, window::WindowMode};
use bevy::window::{PresentMode, WindowResolution, WindowResizeConstraints};

use config::*;
use scenes::{AppState, main_menu};

mod components;
mod config;
mod plugins;
mod resources;
mod scenes;

fn main() {
    let mut window_res = WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT)
        .with_scale_factor_override(2.0);
    window_res.set_scale_factor(2.0);
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                present_mode: PresentMode::AutoNoVsync,
                resolution: window_res,
                title: TITLE.to_string(),
                position: WindowPosition::At(IVec2::new(MONITOR_WIDTH / 4, MONITOR_HEIGHT / 4)),
                resizable: false,
                fit_canvas_to_parent: true,
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_WIDTH,
                    max_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                    max_height: WINDOW_HEIGHT,
                },
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            }),
            ..default()
        }).set(
            ImagePlugin::default_nearest()
        ))
        .add_state::<AppState>()
        .add_plugins(AudioPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(plugins::camera::CameraPlugin)
        .add_plugins(plugins::terrain::TerrainPlugin)
        .add_plugins(plugins::designer::DesignerPlugin)
        .add_plugins(main_menu::MainMenuPlugin)
        .add_loading_state(LoadingState::new(AppState::Loading).continue_to_state(AppState::MainMenu))
        .add_collection_to_loading_state::<_, resources::terrain::ImageAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, resources::designer::ImageAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, resources::audio::AudioAssets>(AppState::Loading)
        .add_collection_to_loading_state::<_, resources::menu::ImageAssets>(AppState::Loading)
        .add_systems(OnEnter(AppState::MainMenu), start_main_menu_music)
        .run();
}

fn start_main_menu_music(audio_assets: Res<resources::audio::AudioAssets>, audio: Res<Audio>) {
    audio.play(audio_assets.main_menu.clone()).looped().with_volume(0.25);
}
