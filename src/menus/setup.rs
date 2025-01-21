use bevy::prelude::*;
use mevy::ui;
use crate::menus::helpers::components::MenuMarker;

pub(crate) fn spawn_menu(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    commands.spawn((ui!((
        display: flex;
        size: 100% 100%;
        flex_direction: column;
        align_items: start;
        justify_content: space_between;
        background: #101014;
        row_gap: 3.5vmax;
        column_gap: 3.5vmax;
        padding: 3vmax 5vmax 15vmax 5vmax;
    )), MenuMarker, Name::new("Menu")));

}