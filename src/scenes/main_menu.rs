use bevy::app::AppExit;
use bevy::{
    prelude::*,
    window::WindowResized,
};
use std::slice::Iter;

use crate::config::WINDOW_WIDTH;
use crate::scenes::AppState;
use crate::resources::menu::ImageAssets;


/// Marker component for the text that displays the current resolution.
#[derive(Component)]
struct ResolutionText;

#[derive(Component, Copy, Clone)]
enum ButtonComponent {
    Levels,
    Settings,
    Quit,
}

impl ButtonComponent {
    pub fn iterator() -> Iter<'static, ButtonComponent> {
        [
            ButtonComponent::Levels,
            ButtonComponent::Settings,
            ButtonComponent::Quit,
        ]
        .iter()
    }
}

#[derive(Resource)]
struct MainMenuSceneData {
    user_interface_root: Entity,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::MainMenu), setup);
        app.add_systems(OnExit(AppState::MainMenu), cleanup);
        app.add_systems(Update, on_resize_system);
        app.add_systems(Update, button_handle_system.run_if(in_state(AppState::MainMenu)));
    }
}

fn setup(mut commands: Commands, menu_assets: Res<ImageAssets>, asset_server: Res<AssetServer>, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();
    let user_interface_root = commands
        .spawn(ImageBundle {
            style: Style {
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..Default::default()
            },
            image: UiImage::new(menu_assets.background.clone()),
            ..Default::default()
        })
        .with_children(|parent| {
            buttons(parent, asset_server, window);
        })
        .id();

    commands.insert_resource(MainMenuSceneData {
        user_interface_root,
    });
}

fn cleanup(mut commands: Commands, main_menu_scene_data: Res<MainMenuSceneData>) {
    commands
        .entity(main_menu_scene_data.user_interface_root)
        .despawn_recursive();
}


fn buttons(root: &mut ChildBuilder, asset_server: Res<AssetServer>, window: &Window) {
    for button in ButtonComponent::iterator() {
        root.spawn( match button {
            ButtonComponent::Quit => ButtonBundle {
                    style: Style {
                        width: Val::Percent(10.0),
                        height: Val::Percent(20.0),
                        justify_content: JustifyContent::Center,
                        position_type: PositionType::Absolute,
                        align_items: AlignItems::Center,
                        align_self: AlignSelf::FlexEnd,
                        left: Val::Percent(6.5),
                        right: Val::Auto,
                        top: Val::Auto,
                        bottom: Val::Percent(-1.0),
                        ..Default::default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..Default::default()
                },
            ButtonComponent::Settings => ButtonBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Percent(20.0),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexEnd,
                    left: Val::Percent(6.5),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Percent(12.0),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            },
            ButtonComponent::Levels => ButtonBundle {
                style: Style {
                    width: Val::Percent(10.0),
                    height: Val::Percent(20.0),
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    align_self: AlignSelf::FlexEnd,
                    left: Val::Percent(6.5),
                    right: Val::Auto,
                    top: Val::Auto,
                    bottom: Val::Percent(25.5),
                    ..Default::default()
                },
                background_color: BackgroundColor(Color::NONE),
                ..Default::default()
            },
        })
        .with_children(|parent| {
            let text: &str = match button {
                ButtonComponent::Quit => "Quit",
                ButtonComponent::Settings => "Settings",
                ButtonComponent::Levels => "Levels",
            };
            parent.spawn((TextBundle {
                text: Text::from_section(
                    text,
                    TextStyle {
                        font: asset_server.load("fonts/DockingBay.ttf"),
                        font_size: 6.0 * ((window.width() / WINDOW_WIDTH) * 2.0),
                        color: Color::WHITE,
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            },
                ResolutionText)
            );
        })
        .insert(button.clone());
    }
}


fn on_resize_system(
    mut q: Query<&mut Text, With<ResolutionText>>,
    mut resize_reader: EventReader<WindowResized>,
) {
    for e in resize_reader.iter() {
        for mut text in q.iter_mut() {
            text.sections[0].style.font_size = 6.0 * ((e.width / WINDOW_WIDTH) * 2.0);
        }
    }
}

fn button_handle_system(
    mut button_query: Query<
        (&Interaction, &ButtonComponent, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut state: ResMut<NextState<AppState>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, button, children) in button_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::None => text.sections[0].style.color = Color::hex("fcffc0").unwrap(),
            Interaction::Hovered => text.sections[0].style.color = Color::hex("ee9c5d").unwrap(),
            Interaction::Pressed => {
                text.sections[0].style.color = Color::hex("c6505a").unwrap();
                match button {
                    ButtonComponent::Levels => state.set(AppState::LevelSelect),
                    ButtonComponent::Settings => state.set(AppState::Options),
                    ButtonComponent::Quit => exit.send(AppExit),
                }
            }
        }
    }
}
