use bevy::prelude::*;
use mevy::ui;
use crate::menus::helpers::{button, DespawnMenu};
use crate::menus::mainmenu::MenuMarker;
use super::states::{MainMenuState, SettingsMenuState};

// This observing code could probably be put in a macro
pub(crate) fn spawn_settings(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>
) {
    if let Ok(menu) = menu.get_single() {
        commands.entity(menu).with_children(|parent| {
            parent.spawn(button("Back")).observe(|
                trigger: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut despawn: EventWriter<DespawnMenu>
            | {
                despawn.send(DespawnMenu(true));
                main_state.set(MainMenuState::Main);
            });

            parent.spawn(button("Video")).observe(|
                trigger: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut despawn: EventWriter<DespawnMenu>
            | {
                despawn.send(DespawnMenu(true));

                settings.set(SettingsMenuState::Video);
            });

            parent.spawn(button("Audio")).observe(|
                trigger: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut despawn: EventWriter<DespawnMenu>
            | {
                despawn.send(DespawnMenu(true));

                settings.set(SettingsMenuState::Audio);
            });

            parent.spawn(button("Controls")).observe(|
                trigger: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut despawn: EventWriter<DespawnMenu>
            | {
                despawn.send(DespawnMenu(true));

                settings.set(SettingsMenuState::Controls);
            });
        });
    }
}

pub(crate) fn spawn_video(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button("Back")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut despawn: EventWriter<DespawnMenu>
        | {
            despawn.send(DespawnMenu(true));
            settings.set(SettingsMenuState::Controls);
        });
    });
}

pub(crate) fn spawn_audio(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button("Back")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut despawn: EventWriter<DespawnMenu>
        | {
            despawn.send(DespawnMenu(true));
            settings.set(SettingsMenuState::Controls);
        });
    });
}

pub(crate) fn spawn_controls(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button("Back")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut despawn: EventWriter<DespawnMenu>
        | {
            despawn.send(DespawnMenu(true));
            settings.set(SettingsMenuState::Controls);
        });
    });
}