use bevy::prelude::*;
use crate::{base_button, ease_button, button, text};
use crate::core::assets::global::GlobalFont;
use crate::menus::states::{MainMenuState, SettingsMenuState};
use crate::menus::helpers::definitions::{BUTTON_WIDTH, BUTTON_HEIGHT, button_padding, BUTTON_NONE_BORDER, TEXT_COLOR, TEXT_SIZE, border_radius, BUTTON_NONE, vmax, color};
use crate::menus::helpers::components::{UiScaleEase, MenuMarker, Ease};

pub fn spawn_settings(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>
) {
    if let Ok(menu) = menu.get_single() {
        commands.entity(menu).with_children(|parent| {
            text!(parent, "Settings", font.handle(), 100.0);
            button!(parent, "Back", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Main);
            });

            button!(parent, "Audio", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                settings.set(SettingsMenuState::Audio);
            });

            button!(parent, "Controls", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                settings.set(SettingsMenuState::Controls);
            });
        });
    }
}

pub fn spawn_audio(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        text!(parent, "Audio", font.handle(), 100.0);
        button!(parent, "Back", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut commands: Commands,
            menu: Query<Entity, With<MenuMarker>>,
        | {
            commands.entity(menu.single()).despawn_descendants();
            settings.set(SettingsMenuState::Settings);
        });
    });
}

pub fn spawn_controls(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        text!(parent, "Controls", font.handle(), 100.0);
        button!(parent, "Back", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut commands: Commands,
            menu: Query<Entity, With<MenuMarker>>,
        | {
            commands.entity(menu.single()).despawn_descendants();
            settings.set(SettingsMenuState::Settings);
        });
    });
}