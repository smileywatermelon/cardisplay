use bevy::prelude::*;
use bevy::window::CursorGrabMode;

pub fn set_grabmode(window: &mut Window, mode: bool) {
    match mode {
        true => {
            window.cursor_options.grab_mode = CursorGrabMode::Confined;
            window.cursor_options.visible = false;
        },
        false => {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
        }
    }
}