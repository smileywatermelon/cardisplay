use bevy::prelude::*;
use mevy::ui;
use crate::core::states::GameState;
use crate::menus::helpers::{button, DespawnMenu};
use crate::menus::states::{MainMenuState};

/// Stores the Node that will contain all the ui
#[derive(Component)]
pub struct MenuMarker;

pub(crate) fn spawn_menu(mut commands: Commands) {
    commands.spawn((ui!((
        display: flex;
        size: 100% 100%;
        flex_direction: column;
        align_items: center;
        justify_content: center;
        background: #242729;
    )), MenuMarker, Name::new("Menu"))).with_children(|parent| {
        // Play Button
        parent.spawn(button("Play")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut commands: Commands,
            menu: Query<Entity, With<MenuMarker>>,
            mut game_state: ResMut<NextState<GameState>>,
        | {
            commands.entity(menu.single()).despawn_recursive();

            game_state.set(GameState::SpawnWorld);
        });

        // Settings Button
        parent.spawn(button("Settings")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut main_state: ResMut<NextState<MainMenuState>>,
            mut despawn: EventWriter<DespawnMenu>
        | {
            despawn.send(DespawnMenu(true));

            main_state.set(MainMenuState::Settings);
        });

            // Exit Button
        parent.spawn(button("Exit")).observe(|
            trigger: Trigger<Pointer<Click>>,
            mut exit: EventWriter<AppExit>
        | {
            exit.send(AppExit::Success);
        });
    });
}