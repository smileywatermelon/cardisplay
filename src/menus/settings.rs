use bevy::prelude::*;
use mevy::ui;
use crate::button;
use crate::menus::mainmenu::MenuMarker;
use super::states::{MainMenuState, SettingsMenuState};

// This observing code could probably be put in a macro
pub(crate) fn spawn_settings(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>
) {
    println!("Spawning Settings");
    if let Ok(menu) = menu.get_single() {
        println!("Got Menu");
        commands.entity(menu).with_children(|parent| {
            parent.spawn(button!("Back")).observe(|
                _: Trigger<Pointer<Click>>,
                mut main_state: ResMut<NextState<MainMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                main_state.set(MainMenuState::Main);
            });

            parent.spawn(button!("Video")).observe(|
                _: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                settings.set(SettingsMenuState::Video);
            });

            parent.spawn(button!("Audio")).observe(|
                _: Trigger<Pointer<Click>>,
                mut settings: ResMut<NextState<SettingsMenuState>>,
                mut commands: Commands,
                menu: Query<Entity, With<MenuMarker>>,
            | {
                commands.entity(menu.single()).despawn_descendants();

                settings.set(SettingsMenuState::Audio);
            });

            parent.spawn(button!("Controls")).observe(|
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

pub(crate) fn spawn_video(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button!("Back")).observe(|
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

pub(crate) fn spawn_audio(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button!("Back")).observe(|
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

pub(crate) fn spawn_controls(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
) {
    commands.entity(menu.single()).with_children(|parent| {
        parent.spawn(button!("Back")).observe(|
            _: Trigger<Pointer<Click>>,
            mut settings: ResMut<NextState<SettingsMenuState>>,
            mut commands: Commands,
            menu: Query<Entity, With<MenuMarker>>,
        | {
            commands.entity(menu.single()).despawn_descendants();
            settings.set(SettingsMenuState::Settings);
        });
        parent.spawn(button!("Controller", Color::BLACK, Color::WHITE));
        parent.spawn(Text::new("Player Controls"));
        parent.spawn(Text::new("Car Controls"));
    });
}