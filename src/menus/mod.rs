mod setup;
pub mod states;
pub mod client;
pub mod main;
pub mod helpers;

use crate::menus::states::{MainMenuState, SettingsMenuState};
use bevy::prelude::*;
use crate::core::states::GameState;
use crate::menus::client::pause::{despawn_pause_menu, spawn_pause_menu};
use crate::menus::helpers::systems::highlight_buttons;
use crate::menus::main::main::spawn_main;
use crate::menus::main::settings::{spawn_audio, spawn_controls, spawn_settings, spawn_video};
use crate::menus::setup::spawn_menu;
use crate::player::states::ClientState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenuState>()
            .add_sub_state::<SettingsMenuState>()

            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            // MainMenu
            .add_systems(OnEnter(MainMenuState::Main), spawn_main)
            // MainMenu - Play - Select

            // MainMenu - Settings
            .add_systems(OnEnter(SettingsMenuState::Settings), spawn_settings)
            .add_systems(OnEnter(SettingsMenuState::Video), spawn_video)
            .add_systems(OnEnter(SettingsMenuState::Audio), spawn_audio)
            .add_systems(OnEnter(SettingsMenuState::Controls), spawn_controls)
            // Client - Pause Menu
            .add_systems(OnEnter(ClientState::Paused), spawn_pause_menu)
            .add_systems(OnExit(ClientState::Paused), despawn_pause_menu)
            // Helpers
            .add_systems(Update, highlight_buttons)
        ;
    }
}
