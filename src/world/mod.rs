mod setup;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::world::setup::spawn_world;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SpawnWorld), spawn_world);
    }
}