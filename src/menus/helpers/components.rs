use bevy::prelude::Component;

#[macro_export]
macro_rules! button {
    ($commands:expr, $text:expr, $font:expr) => {
        $commands.spawn((
            Button,
            Node {
                display: Display::Flex,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: BUTTON_,
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

#[derive(Component)]
pub struct CycleButton {
    strings: Vec<String>,
    current: usize
}

impl CycleButton {
    pub fn new(strings: Vec<String>) -> Self {
        Self {
            strings,
            current: 0
        }
    }
    pub fn get(&self) -> String {
        self.strings[self.current].clone()
    }
    pub fn next(&mut self) -> String {
        let length = self.strings.len() - 1;

        self.current = match self.current {
            _ if self.current == length => 0,
            _ => self.current + 1
        };
        self.get()
    }
}

macro_rules! button_cycle {
    ($commands:expr, $text:expr, $strings:expr, $font:expr) => {
        $commands.spawn((
            Button,
            CycleButton::new($strings),
            Node {
                display: Display::Flex,

            }
        ))
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
            TextColor(text_color()),
        ));
    }
}