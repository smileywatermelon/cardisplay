use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;
use crate::vehicle::setup::{despawn_car, spawn_car};
use crate::vehicle::controls::CarActions;

pub mod parts;
pub mod car;
pub mod controls;
mod setup;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<CarActions>::default())
            .add_systems(OnEnter(SingleplayerState::SpawnVehicles), spawn_car)
            .add_systems(OnExit(GameState::Running), despawn_car)
        ;
    }
}