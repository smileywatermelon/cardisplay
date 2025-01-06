mod player;
mod helpers;
mod controls;
mod physics;

use bevy::prelude::*;
use crate::core::controls::ControlPlugin;
use crate::core::player::PlayerPlugin;
use crate::core::physics::GamePhysicsPlugin;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            GamePhysicsPlugin,
            PlayerPlugin,
            ControlPlugin
        ));
    }
}