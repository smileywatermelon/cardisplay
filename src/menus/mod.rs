mod mainmenu;
pub mod states;
mod settings;
mod helpers;

use crate::menus::states::{MainMenuState, SettingsMenuState};
use bevy::prelude::*;
use crate::core::states::GameState;
use crate::menus::helpers::{despawn_menu, DespawnMenu};
use crate::menus::mainmenu::spawn_menu;
use crate::menus::settings::{spawn_audio, spawn_controls, spawn_settings, spawn_video};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenuState>()
            .add_sub_state::<SettingsMenuState>()
            .add_event::<DespawnMenu>()
            .add_systems(Update, despawn_menu.run_if(in_state(GameState::MainMenu)))
            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            .add_systems(OnEnter(SettingsMenuState::Settings), spawn_settings)
            .add_systems(OnEnter(SettingsMenuState::Video), spawn_video)
            .add_systems(OnEnter(SettingsMenuState::Audio), spawn_audio)
            .add_systems(OnEnter(SettingsMenuState::Controls), spawn_controls)

        ;
    }
}