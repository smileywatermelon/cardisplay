mod controls;
pub mod player;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::player::controls::handle_player_actions;
use crate::player::player::spawn_players;
use crate::vehicle::car::CarState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnPlayers), spawn_players)
            .add_systems(Update, (
                handle_player_actions.run_if(in_state(CarState::OutCar)),
            ).run_if(in_state(GameState::Running)));
    }
}
