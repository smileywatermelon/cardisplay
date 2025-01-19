use bevy::prelude::*;
use crate::menus::helpers::definitions::{button_hover, button_none, button_press};

pub fn highlight_buttons(
    mut buttons: Query<
        (
            &Interaction,
            &mut BackgroundColor,
        ),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background) in buttons.iter_mut() {
        match *interaction {
            Interaction::None => {
                *background = button_none().into();
            },
            Interaction::Hovered => {
                *background = button_hover().into();
            },
            Interaction::Pressed => {
                *background = button_press().into();
            }
        }
    }
}