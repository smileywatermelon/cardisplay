use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::SpawnServer)]
pub enum SingleplayerState {
    #[default]
    SpawnWorld,
    SpawnVehicles,
    Finished
}


/// I will add this after traffic trainer is done
#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates)]
#[source(GameState = GameState::SpawnServer)]
pub enum MultiplayerState {
    #[default]
    SpawnServer,
    SpawnWorld,
    SpawnPlayer,
    SpawnVehicles,
    Finished
}
