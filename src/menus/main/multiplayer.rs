use bevy::prelude::*;
use crate::{base_button, ease_button, button, button_cycle, col, long_button, row, text};
use crate::core::assets::global::GlobalFont;
use crate::core::states::GameState;
use crate::menus::helpers::definitions::{BUTTON_WIDTH, BUTTON_HEIGHT, button_padding, BUTTON_NONE_BORDER, TEXT_COLOR, TEXT_SIZE, border_radius, BUTTON_NONE, vmax, vmin, color};
use crate::menus::helpers::components::{UiScaleEase, MenuMarker, Ease, CycleButton};
use crate::menus::states::MainMenuState;

pub fn spawn_multiplayer_menu(
    mut commands: Commands,
    font: Res<GlobalFont>,
    mut menu: Query<Entity, With<MenuMarker>>,
) {
    if let Ok((menu)) = menu.get_single_mut() {
        commands.entity(menu).with_children(|parent| {
            text!(parent, "Select a Serverr", font.handle(), 100.0);

            col!(parent, JustifyContent::Center, AlignItems::Start).with_children(|parent| {
                long_button!(parent, "localhost:8000", font.handle());
            });

            button!(parent, "Back", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut menu_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                menu_state.set(MainMenuState::Main)
            });
        });
    } else {
        warn!("Couldn't get menu for `spawn_multiplayer_menu`")
    }
}