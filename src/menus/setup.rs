use bevy::prelude::*;
use mevy::ui;
use crate::menus::helpers::MenuMarker;

pub(crate) fn spawn_menu(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.spawn((ui!((
        display: flex;
        size: 100% 100%;
        flex_direction: column;
        align_items: center;
        justify_content: center;
        background: #101014;
        row_gap: 5%;
    )), MenuMarker, Name::new("Menu")));
}