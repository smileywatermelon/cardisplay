use bevy::prelude::*;
use crate::button;
use crate::core::assets::global::GlobalFont;
use crate::core::states::GameState;
use crate::menus::helpers::MenuMarker;
use crate::menus::states::MainMenuState;

pub fn spawn_main(
    mut commands: Commands,
    font: Res<GlobalFont>,
    menu: Query<Entity, With<MenuMarker>>
) {
    commands.entity(menu.single()).with_children(|parent| {
        // Play Button
        button!(parent, "Play", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut main_state: ResMut<NextState<MainMenuState>>,
            mut commands: Commands,
            menu: Query<Entity, With<MenuMarker>>,
        | {
            commands.entity(menu.single()).despawn_descendants();

            main_state.set(MainMenuState::Select);
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
}