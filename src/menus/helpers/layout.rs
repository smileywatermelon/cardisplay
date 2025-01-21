use bevy::prelude::*;

#[macro_export]
macro_rules! col {
    ($commands:expr) => {
        $commands.spawn(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: vmax(3.5),
                row_gap: vmax(3.5),
                ..default()
            }
        )
    };

    ($commands:expr, $justify_content:expr, $align_items:expr) => {
        $commands.spawn(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                justify_content: $justify_content,
                align_items: $align_items,
                column_gap: vmax(3.5),
                row_gap: vmax(3.5),
                ..default()
            }
        )
    }
}

#[macro_export]
macro_rules! row {
    ($commands:expr) => {
        $commands.spawn(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                column_gap: vmax(3.5),
                row_gap: vmax(3.5),
                ..default()
            }
        )
    };

    ($commands:expr, $justify_content:expr, $align_items:expr) => {
        $commands.spawn(
            Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                justify_content: $justify_content,
                align_items: $align_items,
                column_gap: vmax(3.5),
                row_gap: vmax(3.5),
                ..default()
            }
        )
    };
}

#[macro_export]
macro_rules! node {
    ($commands:expr) => {
        $commands.spawn(Node {
            display: Display::Flex,
            ..default()
        })
    };
}