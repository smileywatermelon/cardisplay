use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenuState {
    #[default]
    Main,
    Singleplayer,
    Multiplayer,
    Settings,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(MainMenuState = MainMenuState::Singleplayer)]
pub enum SelectMenuState {
    #[default]
    Name,
    Car,
    Map
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(MainMenuState = MainMenuState::Settings)]
pub enum SettingsMenuState {
    #[default]
    Settings,
    Audio,
    Controls
}