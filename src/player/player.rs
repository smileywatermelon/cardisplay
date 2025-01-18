use crate::player::physics::PlayerControllerBundle;
use avian3d::math::{Quaternion, Scalar, Vector, PI};
use avian3d::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use leafwing_input_manager::prelude::*;
use crate::core::helpers::window::set_grabmode;
use crate::core::states::GameState;
use crate::player::controls::PlayerActions;
use crate::player::states::PlayerCarState;

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    commands.spawn((
        MainPlayer,
        PlayerActions::new(),
        PlayerSettings::default(),
        PlayerControllerBundle::new(Collider::cylinder(1.0, 4.0)),
        Name::new("Player")
    ));

    info!("Player Spawned");

    game_state.set(GameState::SpawnVehicles)
}

pub(crate) fn despawn_player(
    mut commands: Commands,
    player: Query<Entity, With<MainPlayer>>
) {
    commands.entity(player.single()).despawn_recursive();
}

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