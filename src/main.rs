pub mod core;
pub mod vehicle;
mod player;
mod world;
mod menus;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::vehicle::VehiclePlugin;
use crate::core::CorePlugin;
use crate::menus::MenuPlugin;
use crate::player::PlayerPlugin;
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
            MenuPlugin,
            WorldPlugin,
            PlayerPlugin,
            VehiclePlugin
            ))
        .run();

}