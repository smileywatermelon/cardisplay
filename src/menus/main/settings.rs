use bevy::prelude::*;
use crate::button;
use crate::core::assets::global::GlobalFont;
use crate::menus::helpers::MenuMarker;
use crate::menus::states::{MainMenuState, SettingsMenuState};

pub fn spawn_settings(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>
) {
    if let Ok(menu) = menu.get_single() {
        commands.entity(menu).with_children(|parent| {
            button!(parent, "Back", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Main);
            });

            button!(parent, "Video", font.handle()).observe(|
                _: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                settings.set(SettingsMenuState::Video);
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

pub fn spawn_video(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
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

pub fn spawn_audio(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
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