use bevy::prelude::*;
use crate::menus::helpers::components::{Ease, UiScaleEase};
use crate::menus::helpers::definitions::{color, BUTTON_HOVER, BUTTON_NONE, BUTTON_PRESS};

pub fn highlight_buttons(
    mut buttons: Query<
        (
            &mut UiScaleEase,
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (mut ease, interaction, mut background) in buttons.iter_mut() {
        match *interaction {
            Interaction::None => {
                *background = color(BUTTON_NONE).into();
                ease.ease = false;
            },
            Interaction::Hovered => {
                *background = color(BUTTON_HOVER).into();
                ease.ease = true;
            },
            Interaction::Pressed => {
                *background = color(BUTTON_PRESS).into();
            }
        }
    }
}

pub fn ease_buttons(
    mut buttons: Query<(&mut Transform, &mut UiScaleEase), With<Button>>,
) {
    for (mut transform, mut ease) in buttons.iter_mut() {
        let x = match ease.ease {
            true => ease.add(0.0125),
            false => ease.sub(0.0125),
        };

        transform.scale = Vec3::new(x, x, 1.0);
    }
}