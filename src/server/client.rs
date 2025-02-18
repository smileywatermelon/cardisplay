use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, SubStates, Reflect)]
#[source(GameState = GameState::Running)]
pub enum ClientState {
    #[default]
    Running,
    Paused
}