use bevy::prelude::*;
use bevy_common_assets::json::JsonAssetPlugin;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::core::states::GameState;
use crate::vehicle::car::{despawn_car, spawn_car};
use crate::vehicle::controls::CarActions;
use crate::vehicle::parts::engine::EngineFile;

pub mod parts;
pub mod car;
pub mod controls;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(JsonAssetPlugin::<EngineFile>::new(&[""]))
            .add_plugins(InputManagerPlugin::<CarActions>::default())
            .add_systems(OnEnter(GameState::SpawnVehicles), spawn_car)
            .add_systems(OnExit(GameState::Running), despawn_car)

        ;
    }
}