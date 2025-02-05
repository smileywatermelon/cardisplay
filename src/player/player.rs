use crate::player::physics::PlayerControllerBundle;
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::core::states::GameState;
use crate::player::controls::PlayerActions;
use crate::server::states::SingleplayerState;

#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Player;

#[derive(Component)]
#[require(Player, PlayerSettings)]
/// This will mark the player playing on the running device
pub struct MainPlayer;

#[derive(Component)]
pub struct PlayerSettings {
    /// Should be `0.0` - `100.0`
    ///
    /// Default: `75.0`
    pub sensitivity: f32,
    /// Movement speed multiplier
    ///
    /// Default: `10.0`
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 75.0,
            speed: 10.0,
        }
    }
}