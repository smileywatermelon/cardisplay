pub mod core;
pub mod vehicle;
pub mod player;
pub mod world;
pub mod menus;
mod debug;
pub mod server;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::vehicle::VehiclePlugin;
use crate::core::CorePlugin;
use crate::debug::DebugPlugin;
use crate::menus::MenuPlugin;
use crate::player::PlayerPlugin;
use crate::server::ServerPlugin;
use crate::world::WorldPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "cardisplay".to_string(),
                    position: WindowPosition::Centered(MonitorSelection::Primary),
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::DX12),
                    ..default()
                }),
                ..default()
            })
        )
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins((
            CorePlugin,
            DebugPlugin,
            MenuPlugin,

            // Game
            ServerPlugin,
            WorldPlugin,
            PlayerPlugin,
            VehiclePlugin
            ))
        .run();

}