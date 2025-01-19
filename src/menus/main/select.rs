use bevy::prelude::*;
use crate::button;
use crate::core::assets::global::GlobalFont;
use crate::menus::helpers::MenuMarker;

pub fn spawn_select(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>,
) {
    if let Ok(menu) = menu.get_single() {
        commands.entity(menu).with_children(|parent| {
            button!(parent, "Play", font.handle());
            button!(parent, "Car: Honda Civic Type R", font.handle());

        });
    } else {
        warn!("Couldn't get menu for `spawn_select`")
    }
}