use bevy::prelude::*;
use crate::menus::helpers::components::{CycleButton, UiScaleEase};
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
macro_rules! ease_button {
    ($commands:expr, $text:expr, $font:expr, $width:expr, $height:expr) => {
        base_button!($commands, $text, $font, $width, $height)
            .insert(UiScaleEase::new(1.0..1.1))
    }
}

#[macro_export]
macro_rules! button {
    ($commands:expr, $text:expr, $font:expr) => {
        ease_button!($commands, $text, $font, vmax(BUTTON_WIDTH), vmax(BUTTON_HEIGHT))
    };
}

#[macro_export]
macro_rules! short_button {
    ($commands:expr, $text:expr, $font:expr) => {
        base_button!($commands, $text, $font, vmax(2.5), vmax(BUTTON_HEIGHT))
    };
}

#[macro_export]
macro_rules! long_button {
    ($commands:expr, $text:expr, $font:expr) => {
        ease_button!($commands, $text, $font, vmax(90.0), vmax(BUTTON_HEIGHT))
    };
}

#[macro_export]
macro_rules! button_cycle {
    ($commands:expr, $strings:expr, $font:expr) => {

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