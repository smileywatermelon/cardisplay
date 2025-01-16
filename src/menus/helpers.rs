use bevy::prelude::*;
use crate::menus::mainmenu::MenuMarker;


#[macro_export]
macro_rules! button {
    ($text:expr) => {
        (
            Button,
            Text::new($text.to_string()),
            Node {
                display: Display::Flex,
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                padding: UiRect::all(Val::Percent(1.0)),
                border: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(20, 22, 23)),
            BorderColor(Color::WHITE)
        )
    };

    ($text:expr, $background:expr, $border:expr) => {
        (
            Button,
            Text::new($text.to_string()),
            Node {
                display: Display::Flex,
                width: Val::Percent(20.0),
                height: Val::Percent(5.0),
                padding: UiRect::all(Val::Percent(1.0)),
                border: UiRect::all(Val::Percent(1.0)),
                ..default()
            },
            BackgroundColor($background),
            BorderColor($border)
        )
    }
}