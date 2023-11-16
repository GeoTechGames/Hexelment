use bevy::prelude::*;

use crate::components::player::PlayerComponent;
use crate::scenes::AppState;


#[derive(Component)]
pub struct UICamera;

#[derive(Component)]
pub struct SimCamera;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_user_interface_camera);
        app.add_systems(Startup, spawn_2d_camera);

        app.add_systems(Update, camera_follow
            .run_if(in_state(AppState::Simulation))
            .run_if(in_state(AppState::Results))
        );

        app.add_systems(OnExit(AppState::Simulation), reset_camera);
    }
}

fn spawn_user_interface_camera(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(Name::new("UICamera"))
        .insert(UICamera);
}

fn spawn_2d_camera(mut commands: Commands) {
    let mut camera = Camera2dBundle::default();
    camera.camera.order = 1;

    commands
        .spawn(camera)
        .insert(SimCamera)
        .insert(Name::new("SimCamera"));
}

fn camera_follow(
    player_query: Query<&Transform, With<PlayerComponent>>,
    mut camera_query: Query<&mut Transform, (Without<PlayerComponent>, With<SimCamera>)>,
) {
    let player_transform = player_query.single();
    let mut camera_transform = camera_query.single_mut();

    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

fn reset_camera(mut camera_query: Query<&mut Transform, With<SimCamera>>) {
    let mut camera_transform = camera_query.single_mut();
    camera_transform.translation.x = 0.0;
    camera_transform.translation.y = 0.0;
}
