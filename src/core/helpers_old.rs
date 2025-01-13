
use bevy::asset::Handle;
use bevy::prelude::{Font, Text, TextFont, Window};
use bevy::window::CursorGrabMode;

pub fn toggle_grabmode(window: &mut Window) {
    match window.cursor_options.grab_mode {
        CursorGrabMode::None => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        },
        _ => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}

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