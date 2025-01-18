use bevy::prelude::*;
use crate::menus::states::{MainMenuState, SettingsMenuState};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum GameState {
    #[default]
    /// When game is initializing
    Loading,
    /// MainMenu
    MainMenu,
    /// Initialize server
    SpawnServer,
    /// Spawn the world, then go to Players
    SpawnWorld,
    /// Spawn the players, then go to vehicles
    SpawnPlayers,
    /// Spawn the vehicles, then go to Running
    SpawnVehicles,
    /// Stay here indefinitely until game is paused, then go to Paused
    Running,
}

#[derive(Component)]
pub struct CurrentStateMarker;

pub(crate) fn spawn_current_state(
    mut commands: Commands,
) {
    commands.spawn((
        Text::new("State: "),
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
        GameState::SpawnWorld => "SpawnWorld",
        GameState::SpawnPlayers => "SpawnPlayers",
        GameState::SpawnVehicles => "SpawnVehicles",
        GameState::Running => "Running",
    };
    if let Ok(mut current_state) = current_state.get_single_mut() {
        current_state.0 = game.to_string();
    }
}