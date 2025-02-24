use bevy::prelude::*;
use bevy::tasks::futures_lite::StreamExt;
use leafwing_input_manager::prelude::*;
use crate::server::client::ClientState;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum MenuControls {
    TogglePause
}

pub fn spawn_menu_controls(mut commands: Commands) {
    let input_map = InputMap::default()
        .with(MenuControls::TogglePause, KeyCode::Escape);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub fn handle_menu_controls(
    pause: Query<&ActionState<MenuControls>>,
    client_state: Res<State<ClientState>>,
    mut next_state: ResMut<NextState<ClientState>>
) {
    let pause = pause.single();

    if pause.just_pressed(&MenuControls::TogglePause) {
        let state = match client_state.get() {
            ClientState::Running => ClientState::Paused,
            ClientState::Paused => ClientState::Running,
        };

        next_state.set(state);
    }
}