use bevy::prelude::States;

pub mod main_menu;

#[derive(Debug, Clone, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    Options,
    LevelSelect,
    Designer,
    Simulation,
    Results,
}
