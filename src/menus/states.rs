use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenuState {
    #[default]
    Main,
    Settings,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(MainMenuState = MainMenuState::Settings)]
pub enum SettingsMenuState {
    #[default]
    Settings,
    Video,
    Audio,
    Controls
}