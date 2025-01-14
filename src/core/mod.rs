mod physics;
mod camera;
pub mod helpers;
pub mod states;
pub mod xinput;

use bevy::prelude::*;
use crate::core::camera::spawn_camera;
use crate::core::physics::GamePhysicsPlugin;
use crate::core::states::{update_current_state, spawn_current_state, GameState};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .add_plugins((
                GamePhysicsPlugin,
            ))
            .add_systems(Startup, (
                spawn_camera,
                spawn_current_state
                ))
            .add_systems(Update, update_current_state);
    }
}