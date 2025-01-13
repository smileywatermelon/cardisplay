use bevy::prelude::*;
use mevy::ui;
use crate::core::states::GameState;

/// Not a traditional marker, but just stores the Node that will be used to
/// render all menus on
#[derive(Component)]
pub struct MenuMarker;

pub(crate) fn init_menu(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>
) {
    commands.spawn((Node {
        display: Display::Block,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    }, MenuMarker, Name::new("Menu"))).with_children(|parent| {
        parent.spawn((
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb_u8(102, 119, 128))
        )).with_children(|parent| {
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(20.0),
                    padding: UiRect::vertical(Val::Percent(0.5)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(125, 168, 181)),
                BorderColor(Color::WHITE),
                Text::new("Play")
            ));
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(20.0),
                    padding: UiRect::vertical(Val::Percent(0.5)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(125, 168, 181)),
                BorderColor(Color::WHITE),
                Text::new("Settings")
            ));
            parent.spawn((
                Button,
                Node {
                    width: Val::Percent(20.0),
                    padding: UiRect::vertical(Val::Percent(0.5)),
                    ..default()
                },
                BackgroundColor(Color::srgb_u8(125, 168, 181)),
                BorderColor(Color::WHITE),
                Text::new("Exit")
            ));
        });
    });
    println!("Spawned thingy");

    game_state.set(GameState::SpawnPlayers);
}