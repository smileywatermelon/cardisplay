mod setup;

use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;
use crate::world::setup::{despawn_world, spawn_singleplayer_world};

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(SingleplayerState::SpawnWorld), spawn_singleplayer_world)
            .add_systems(OnExit(GameState::Running), despawn_world)
        ;
    }
}