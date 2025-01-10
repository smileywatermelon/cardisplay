use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::cars::car::{spawn_car, spawn_engine_audio, update_engine_noise, update_wheel_rpm};
use crate::cars::controls::{handle_car_actions, spawn_car_actions, CarActions};
use crate::cars::debug::{spawn_car_debug, update_car_debug};

pub mod car;
pub mod parts;
pub mod wheels;
mod controls;
mod debug;

pub struct CarPlugin;

impl Plugin for CarPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<CarActions>::default())
            .add_systems(Startup, (
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
                ));
    }
}