mod setup;
pub mod states;
pub mod client;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::client::ClientState;
use crate::server::setup::{set_running, spawn_singleplayer};
use crate::server::states::{MultiplayerState, SingleplayerState};

pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<SingleplayerState>()
            .add_sub_state::<MultiplayerState>()
            .add_sub_state::<ClientState>()
            .add_systems(OnEnter(GameState::SpawnServer), spawn_singleplayer)
            .add_systems(OnEnter(SingleplayerState::Finished), set_running)
        ;
    }
}