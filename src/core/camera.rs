use bevy::prelude::*;

pub(crate) fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera3d::default());
}