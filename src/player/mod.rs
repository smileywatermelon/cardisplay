mod controls;
pub mod player;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::player::player::spawn_players;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnPlayers), spawn_players);
    }
}
