use bevy::prelude::*;

/// `vmax` function
pub fn vmax(value: f32) -> Val { Val::VMax(value) }
pub fn vmin(value: f32) -> Val { Val::VMin(value) }
pub fn vh(value: f32) -> Val { Val::Vh(value) }
pub fn vw(value: f32) -> Val { Val::Vw(value) }

pub fn color<S: ToString>(hex: S) -> Color { Color::Srgba(Srgba::hex(hex.to_string()).unwrap()) }

pub const BACKGROUND: &str = "101014";

pub const TEXT_COLOR: &str = "ebeded";
pub const TEXT_SIZE: f32 = 40.0;

pub const BUTTON_NONE: &str = "22222a";
pub const BUTTON_NONE_BORDER: &str = "30303b";
pub const BUTTON_HOVER: &str = "292933";
pub const BUTTON_HOVER_BORDER: &str = "373744";
pub const BUTTON_PRESS: &str = "30303b";
pub const BUTTON_PRESS_BORDER: &str = "3e3e4c";

pub const BUTTON_WIDTH: f32 = 20.0;
pub const BUTTON_HEIGHT: f32 = 5.0;

pub fn button_padding() -> UiRect {
    UiRect::all(vmax(1.0)).with_top(vmax(1.5))
}

pub fn border_radius() -> BorderRadius {
    BorderRadius::all(vmax(1.0))
}