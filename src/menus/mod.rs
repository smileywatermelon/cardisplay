mod setup;
pub mod states;
pub mod client;
pub mod main;
pub mod helpers;
mod controls;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::menus::client::pause::{despawn_pause_menu, spawn_pause_menu};
use crate::menus::controls::{handle_menu_controls, spawn_menu_controls};
use crate::menus::helpers::systems::{ease_buttons, highlight_buttons};
use crate::menus::main::main::spawn_main;
use crate::menus::main::multiplayer::spawn_multiplayer_menu;
use crate::menus::main::singleplayer::spawn_singleplayer_menu;
use crate::menus::main::settings::{spawn_settings, spawn_audio, spawn_controls};
use crate::menus::setup::spawn_menu;
use crate::menus::states::{MainMenuState, SettingsMenuState};
use crate::server::client::ClientState;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<MainMenuState>()
            .add_sub_state::<SettingsMenuState>()

            .add_systems(OnEnter(GameState::MainMenu), spawn_menu)
            // MainMenu
            .add_systems(OnEnter(MainMenuState::Main), spawn_main)
            // MainMenu - Play - Singleplayer
            .add_systems(OnEnter(MainMenuState::Singleplayer), spawn_singleplayer_menu)
            // MainMenu - Play - Multiplayer
            .add_systems(OnEnter(MainMenuState::Multiplayer), spawn_multiplayer_menu)
            // MainMenu - Settings
            .add_systems(OnEnter(SettingsMenuState::Settings), spawn_settings)
            .add_systems(OnEnter(SettingsMenuState::Audio), spawn_audio)
            .add_systems(OnEnter(SettingsMenuState::Controls), spawn_controls)
            // Client - Pause Menu
            .add_systems(OnEnter(ClientState::Paused), spawn_pause_menu)
            .add_systems(OnExit(ClientState::Paused), despawn_pause_menu)
            .add_systems(Startup, spawn_menu_controls)
            .add_systems(Update, handle_menu_controls.run_if(in_state(GameState::Running)))
            // Helpers
            .add_systems(Update, highlight_buttons)
            .add_systems(Update, ease_buttons)
        ;
    }
}
