use crate::player::GameState;
use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, Reflect)]
pub enum PlayerCarState {
    InCar,
    #[default]
    OutCar
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, Reflect)]
pub enum PlayerMultiplayerSetup {
    #[default]
    Singleplayer,
    Multiplayer
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates, Reflect)]
#[source(GameState = GameState::Running)]
pub enum ClientState {
    #[default]
    Running,
    Paused
}