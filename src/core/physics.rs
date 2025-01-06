use avian3d::prelude::*;
use bevy::prelude::*;

pub struct GamePhysicsPlugin;
impl Plugin for GamePhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default());
    }
}