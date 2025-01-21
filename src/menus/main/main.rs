use bevy::prelude::*;
use crate::{base_button, button, col, text};
use crate::core::assets::global::GlobalFont;
use crate::menus::states::MainMenuState;
use crate::menus::helpers::definitions::{BUTTON_WIDTH, BUTTON_HEIGHT, button_padding, BUTTON_NONE_BORDER, TEXT_COLOR, TEXT_SIZE, border_radius, BUTTON_NONE, vmax, color};
use crate::menus::helpers::components::{UiScaleEase, MenuMarker, Ease};


pub fn spawn_main(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>
) {
    commands.entity(menu.single()).with_children(|parent| {
        text!(parent, "Traffic Trainer", font.handle(), 100.0);

        col!(parent, JustifyContent::Center, AlignItems::Start).with_children(|parent| {
            // Play Button
            button!(parent, "Singleplayer", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Singleplayer);
            });

            button!(parent, "Multiplayer", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Multiplayer);
            });

            // Settings Button
            button!(parent, "Settings", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Settings);
            });

            // Exit Button
            button!(parent, "Exit", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut exit: EventWriter<AppExit>
            | {
                exit.send(AppExit::Success);
            });
        });
    });
}