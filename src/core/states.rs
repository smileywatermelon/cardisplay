use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    /// When game is initializing
    Loading,
    /// MainMenu
    MainMenu,
    /// Initialize server
    SpawnServer,
    /// Game is running
    Running,
}

#[derive(Component)]
pub struct CurrentStateMarker;

pub(crate) fn spawn_current_state(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("State: "),
        Name::new("StateDebug"),
        CurrentStateMarker
    ));
}

pub(crate) fn update_current_state(
    game: Res<State<GameState>>,
    mut current_state: Query<&mut Text, With<CurrentStateMarker>>
) {
    let game = match game.get() {
        GameState::Loading => "Loading",
        GameState::MainMenu => "MainMenu",
        GameState::SpawnServer => "SpawnServer",
        GameState::Running => "Running",
    };
    if let Ok(mut current_state) = current_state.get_single_mut() {
        current_state.0 = game.to_string();
    }
}