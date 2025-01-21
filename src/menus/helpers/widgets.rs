use bevy::prelude::*;
use crate::menus::helpers::components::UiScaleEase;
use super::definitions::{BUTTON_WIDTH, BUTTON_HEIGHT, button_padding, BUTTON_NONE_BORDER, TEXT_COLOR, TEXT_SIZE, border_radius, BUTTON_NONE};

#[macro_export]
macro_rules! base_button {
    ($commands:expr, $text:expr, $font:expr, $width:expr, $height:expr) => {
        $commands.spawn((
            Button,
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: $width,
                height: $height,
                padding: button_padding(),
                ..default()
            },
            BackgroundColor(color(BUTTON_NONE)),
            border_radius(),
            BorderColor(color(BUTTON_NONE_BORDER)),
            BoxShadow {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: vmax(0.0),
                y_offset: vmax(0.0),
                spread_radius: Val::Px(5.0),
                blur_radius: Val::Px(5.0),
            }
        )).with_children(|parent| {
            parent.spawn((
                Text::new($text.to_string()),
                TextFont {
                    font: $font,
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(color(TEXT_COLOR)),
            ));
        })
    };
}

#[macro_export]
macro_rules! button {
    ($commands:expr, $text:expr, $font:expr) => {
        base_button!($commands, $text, $font, vmax(BUTTON_WIDTH), vmax(BUTTON_HEIGHT))
    };
}

#[macro_export]
macro_rules! long_button {
    ($commands:expr, $text:expr, $font:expr) => {
        base_button!($commands, $text, $font, vmax(90.0), vmax(BUTTON_HEIGHT))
    };

    ($commands:expr, $text:expr, $font:expr, $width:expr) => {
        base_button!($commands, $text, $font, vmax($width), vmax(BUTTON_HEIGHT))
    }
}

#[macro_export]
macro_rules! button_cycle {
    ($commands:expr, $strings:expr, $font:expr) => {
        $commands.spawn((
            Button,
            CycleButton::new($strings),
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: vmax(BUTTON_WIDTH),
                height: vmax(BUTTON_HEIGHT),
                padding: button_padding(),
                ..default()
            },
            BackgroundColor(color(BUTTON_NONE)),
            border_radius(),
            BorderColor(color(BUTTON_NONE_BORDER)),
            BoxShadow {
                color: Color::BLACK.with_alpha(0.5),
                x_offset: vmax(0.0),
                y_offset: vmax(0.0),
                spread_radius: Val::Px(5.0),
                blur_radius: Val::Px(5.0),
            }
        )).with_children(|parent| {
            parent.spawn((
                Text::new($strings[0].clone()),
                TextFont {
                    font: $font,
                    font_size: TEXT_SIZE,
                    ..default()
                },
                TextColor(color(TEXT_COLOR)),
            ));
        })

    }
}

#[macro_export]
macro_rules! text {
    ($commands:expr, $text:expr, $font:expr) => {
        $commands.spawn((
            Text::new($text.to_string()),
            TextFont {
                font: $font,
                font_size: TEXT_SIZE,
                ..default()
            },
            TextColor(color(TEXT_COLOR)),
        ))
    };

    ($commands:expr, $text:expr, $font:expr, $size:expr) => {
        $commands.spawn((
            Text::new($text.to_string()),
            TextFont {
                font: $font,
                font_size: $size,
                ..default()
            },
            TextColor(color(TEXT_COLOR)),
        ))
    }
}