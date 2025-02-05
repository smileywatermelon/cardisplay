use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;

pub(crate) fn spawn_singleplayer(
    mut singleplayer: ResMut<NextState<SingleplayerState>>,
) {
    singleplayer.set(SingleplayerState::SpawnWorld);
}

pub(crate) fn set_running(
    mut game_state: ResMut<NextState<GameState>>,
) {
    game_state.set(GameState::Running);
}

/// I will add this after traffic trainer is done
pub(crate) fn spawn_multiplayer(
    mut commands: Commands,
) {
    todo!()
}