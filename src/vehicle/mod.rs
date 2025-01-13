use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::core::states::GameState;
use crate::vehicle::car::{spawn_car, spawn_engine_audio, update_engine_noise, update_wheel_rpm};
use crate::vehicle::controls::{handle_car_actions, spawn_car_actions, CarActions};
use crate::vehicle::debug::{spawn_car_debug, update_car_debug};

pub mod parts;
mod car;
mod controls;
mod debug;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<CarActions>::default())
            .add_systems(OnEnter(GameState::SpawnVehicles), (
                spawn_car,
                spawn_car_actions,
                spawn_engine_audio,
                spawn_car_debug
            ))
            .add_systems(Update, (
                update_engine_noise,
                handle_car_actions,
                update_wheel_rpm,
                update_car_debug
            ).run_if(in_state(GameState::Running)));
    }
}