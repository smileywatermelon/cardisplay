mod physics;
mod camera;
pub mod helpers;
pub mod states;
pub mod handles;

use bevy::prelude::*;
use bevy_rand::prelude::*;

use crate::core::camera::spawn_camera;
use crate::core::handles::insert_font_handle;
use crate::core::physics::GamePhysicsPlugin;
use crate::core::states::{update_current_state, spawn_current_state, GameState};

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(GameState::default())
            .add_plugins((
                GamePhysicsPlugin,
                EntropyPlugin::<WyRand>::default(),
            ))
            .add_systems(Startup, (
                spawn_camera,
                spawn_current_state,
                insert_font_handle
                ))
            .add_systems(Update, update_current_state);
    }
}