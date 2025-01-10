pub mod core;
pub mod cars;

use bevy::prelude::*;
use bevy::render::RenderPlugin;
use bevy::render::settings::{Backends, RenderCreation, WgpuSettings};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use crate::cars::CarPlugin;
use crate::core::CorePlugin;

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
            CarPlugin
            ))
        .run();

}