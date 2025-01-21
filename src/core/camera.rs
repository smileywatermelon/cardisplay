use bevy::prelude::*;
use crate::core::states::GameState;

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}