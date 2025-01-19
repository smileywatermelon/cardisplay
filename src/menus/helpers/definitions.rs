use bevy::prelude::*;

#[macro_export]
macro_rules! color_fn {
    ($name:ident, $r:expr, $g:expr, $b:expr) => {
        pub fn $name() -> Color {
            Color::srgb_u8(r, g, b)
        }
    };
    ($name:ident, $r:expr, $g:expr, $b:expr, $a:expr) => {
        pub fn $name() -> Color {
            Color::srgba_u8($r, $g, $b, $a)
        }
    }
}

/// `vmax` macro, for consts mostly
#[macro_export]
macro_rules! vmax {
    ($name:ident, $value:expr) => {
        pub fn $name() -> Val {
            vmax($value)
        }
    };
}

/// `vmax` function
pub fn vmax(value: f32) -> Val {
    Val::VMax(value)
}

color_fn!(background, 16, 16, 20);

color_fn!(text, 235, 237, 237);
const TEXT_SIZE: f32 = 40.0;

color_fn!(button_none, 34, 34, 42);
color_fn!(button_hover, 41, 41, 51);
color_fn!(button_press, 48, 48, 59);

vmax!(button_width, 20.0);
vmax!(button_height, 5.0);

fn button_padding() -> UiRect {
    UiRect::all(vmax(1.0)).with_top(vmax(1.5))
}