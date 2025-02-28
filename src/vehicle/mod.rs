use bevy::prelude::*;
use leafwing_input_manager::plugin::InputManagerPlugin;
use crate::core::states::GameState;
use crate::server::client::ClientState;
use crate::server::states::SingleplayerState;
use crate::vehicle::car::handle_car;
use crate::vehicle::setup::{despawn_car, spawn_car};
use crate::vehicle::controls::{handle_camera, handle_controls, CarActions};
use crate::vehicle::parts::prelude::*;

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

            .add_systems(Update, (
                handle_camera,
                handle_controls,
                handle_car,
                )
                .chain()
                .run_if(in_state(ClientState::Running).and(in_state(GameState::Running))))

            // Inspector registers
            .register_type::<Engine>()
            .register_type::<EngineSetup>()
            .register_type::<Transmission>()
        ;

    }
}