use bevy::prelude::*;
use controls::CarActionsKeyboard;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::core::states::GameState;
use crate::vehicle::car::{spawn_car, spawn_engine_audio, update_engine_noise, update_wheel_rpm, CarState};
use crate::vehicle::controls::{handle_car_actions, handle_gamepad_pedals, handle_kb_pedals, spawn_car_actions, CarActions, CarPedalMode};
use crate::vehicle::debug::{spawn_car_debug, update_car_debug};

pub mod parts;
pub mod car;
pub mod controls;
mod debug;

pub struct VehiclePlugin;

impl Plugin for VehiclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<CarActions>::default())
            .insert_resource(CarActionsKeyboard::default())
            .insert_state(CarState::default())
            .insert_state(CarPedalMode::default())
            .add_systems(OnEnter(GameState::SpawnVehicles), (
                spawn_car,
                spawn_car_actions,
                spawn_engine_audio,
            ))
            .add_systems(Update, (
                update_engine_noise,
                handle_car_actions,
                handle_gamepad_pedals.run_if(in_state(CarPedalMode::Controller)),
                handle_kb_pedals.run_if(in_state(CarPedalMode::Keyboard)),
                update_wheel_rpm,
            ).run_if(in_state(GameState::Running).and(in_state(CarState::InCar))));
    }
}