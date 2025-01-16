use bevy::prelude::*;

pub fn text<S: Into<String>>(text: S, font_size: f32, font: Option<Handle<Font>>) -> (Text, TextFont) {
    (
        Text::new(text),
        TextFont {
            font: font.unwrap_or_default(),
            font_size,
            ..Default::default()
        }
    )
}