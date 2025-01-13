use bevy::prelude::*;
use crate::core::states::GameState;

pub(crate) fn spawn_camera(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(Camera3d::default());

    game_state.set(GameState::MainMenu)
}