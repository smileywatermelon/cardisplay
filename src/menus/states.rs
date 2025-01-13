use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::MainMenu)]
pub enum MainMenu {
    #[default]
    Main,
    Settings,
    Exit,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(MainMenu = MainMenu::Settings)]
pub enum SettingsMenu {
    #[default]
    Settings,
    Video,
    Audio,
    Controls
}