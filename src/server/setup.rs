use bevy::prelude::*;
use crate::core::states::GameState;

pub fn spawn_singleplayer(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::Running)
}

pub fn spawn_multiplayer(
    mut commands: Commands,
) {
    todo!()
}