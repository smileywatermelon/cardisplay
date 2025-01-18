use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use crate::menus::mainmenu::MenuMarker;

#[macro_export]
macro_rules! button {
    ($commands:expr, $text:expr, $font:expr) => {
        $commands.spawn((
            Button,
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::VMax(20.0),
                height: Val::VMax(5.0),
                padding: UiRect::all(Val::VMax(1.0)).with_top(Val::VMax(1.5)),
                ..default()
            },
            BackgroundColor(Color::srgb_u8(20, 22, 23)),
            BorderRadius::all(Val::Px(10.0)),
            BorderColor(Color::srgb_u8(20, 20, 20)),
            BoxShadow {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: Val::Px(0.0),
                y_offset: Val::Px(0.0),
                spread_radius: Val::Px(5.0),
                blur_radius: Val::Px(5.0),
            }
        )).with_children(|parent| {
            parent.spawn((
                Text::new($text.to_string()),
                TextFont {
                    font: $font,
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::srgb_u8(235, 237, 237)),
            ));
        })
    };
}

type ConstColor = (u8, u8, u8);

const BUTTON_NONE: ConstColor = (34, 34, 42);
const BUTTON_HOVER: ConstColor = (41, 41, 51);
const BUTTON_PRESS: ConstColor = (48, 48, 59);

fn color(color: ConstColor) -> Color {
    Color::srgb_u8(color.0, color.1, color.2)
}

pub(crate) fn highlight_buttons(
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background) in buttons.iter_mut() {
        match *interaction {
            Interaction::None => {
                *background = color(BUTTON_NONE).into();
            },
            Interaction::Hovered => {
                *background = color(BUTTON_HOVER).into();
            },
            Interaction::Pressed => {
                *background = color(BUTTON_PRESS).into();
            }
        }
    }
}

#[macro_export]
macro_rules! text {
    ($commands:expr, $text:expr, $font:expr) => {
        $commands.spawn((
            Text::new($text.to_string()),
            TextFont {
                font: $font,
                font_size: 40.0,
                ..default()
            },
            TextColor(Color::srgb_u8(235, 237, 237)),
        ));
    }
}