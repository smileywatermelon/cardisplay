mod controls;
pub mod player;
mod physics;
pub mod states;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::core::states::GameState;
use crate::player::controls::{disable_look, handle_player_actions, handle_player_look, handle_player_move, toggle_pause, PlayerActions};
use crate::player::player::{despawn_player, spawn_player};
use crate::player::states::{ClientState, PlayerCarState};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_sub_state::<ClientState>()
            .insert_state(PlayerCarState::OutCar)
            .add_plugins(InputManagerPlugin::<PlayerActions>::default())
            .add_systems(OnEnter(GameState::SpawnPlayers), spawn_player)
            .add_systems(Update, (
                (
                    handle_player_look,
                    handle_player_move,
                ).run_if(in_state(ClientState::Running)),
                toggle_pause
            ).run_if(in_state(GameState::Running)))
            .add_systems(OnExit(GameState::Running), despawn_player)
        ;
    }
}
