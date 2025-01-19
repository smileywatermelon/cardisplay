use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use mevy::ui;
use crate::{button, text};
use crate::core::assets::GlobalFont;
use crate::core::helpers::prelude::set_grabmode;
use crate::core::states::GameState;
use crate::menus::setup::MenuMarker;
use crate::player::states::ClientState;

pub(crate) fn spawn_pause_menu(
    mut commands: Commands,
    font: Res<GlobalFont>
) {
    commands.spawn((ui!((
        display: flex;
        size: 30% 100%;
        flex_direction: column;
        align_items: start;
        padding: 5%;
        row_gap: 5%;
        background: #101014;
    )), MenuMarker, Name::new("PauseMenu"))).with_children(|parent| {
        button!(parent, "Resume", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut client_state: ResMut<NextState<ClientState>>
        | {
            client_state.set(ClientState::Running);
        });
        text!(parent, "Settings", font.handle());
        button!(parent, "Settings", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut game_state: ResMut<NextState<GameState>>
        | {

        });
        button!(parent, "Exit", font.handle()).observe(|
            _: Trigger<Pointer<Click>>,
            mut game_state: ResMut<NextState<GameState>>
        | {
            game_state.set(GameState::MainMenu);
        });
    });
}

pub(crate) fn despawn_pause_menu(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    commands.entity(menu.single()).despawn_recursive();

    let mut window = window.single_mut();
    set_grabmode(&mut window, true);
}