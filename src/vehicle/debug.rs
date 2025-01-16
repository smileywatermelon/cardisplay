use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use crate::vehicle::parts::prelude::*;

use super::controls::{CarActions, CarActionsKeyboard};

#[derive(Component)]
pub struct CarDebugMarker(pub usize);

fn text<S: ToString>(text: S, id: usize) -> (Text, CarDebugMarker) {
    (
        Text::new(text.to_string()),
        CarDebugMarker(id),
    )
}

pub(crate) fn spawn_car_debug(mut commands: Commands) {
    let cnode = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        ..default()
    };

    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(108, 121, 128)),
    )).with_children(|parent| {
        parent.spawn(cnode.clone()).with_children(|parent| {
            parent.spawn(Text::new("Engine"));
            parent.spawn(text("RPM", 0));
            parent.spawn(text("THR", 1));
            parent.spawn(text("GPT", 2));
            parent.spawn(text("KBT", 3));
        });
        parent.spawn(cnode.clone()).with_children(|parent| {
            parent.spawn(Text::new("Transmission"));
            parent.spawn(text("Selected", 4));
        });
        parent.spawn(cnode).with_children(|parent| {
            parent.spawn(Text::new("Wheels"));
            parent.spawn(text("Brake", 5));
            parent.spawn(text("GPB", 6));
            parent.spawn(text("KBB", 7));
            parent.spawn(text("TL", 8));
            parent.spawn(text("TR", 9));
            parent.spawn(text("BL", 10));
            parent.spawn(text("BR", 11));
        });
    });
}

pub(crate) fn update_car_debug(
    mut car_debug: Query<(&mut Text, &CarDebugMarker)>,
    car: Query<(&Engine, &Transmission, &Wheels, &Brakes)>,
    actions: Query<&ActionState<CarActions>>,
    kb_actions: Res<CarActionsKeyboard>
) {
    let (engine, transmission, wheels, brakes) = car.single();
    let actions = actions.single();

    for (mut text, marker) in car_debug.iter_mut() {
        match marker.0 {
            0 => {
                text.0 = format!("RPM: {:.0}", engine.rpm());
            },
            1 => {
                text.0 = format!("Throttle: {:.2}", engine.throttle());
            },
            2 => {
                text.0 = format!("Gamepad: {:.2}", actions.clamped_value(&CarActions::Throttle))
            },
            3 => {
                text.0 = format!("Keyboard: {:.2}", kb_actions.throttle);
            },
            4 => {
                text.0 = transmission.gear_string();
            },
            5 => {
                text.0 = format!("Brakes: {}", brakes.pressure());
            },
            6 => {
                text.0 = format!("Gamepad: {:.2}", actions.clamped_value(&CarActions::Brake))
            },
            7 => {
                text.0 = format!("Keyboard: {:.2}", kb_actions.brake)
            },
            8 => {
                text.0 = format!("TL: {}", wheels.top_left.rpm)
            },
            9 => {
                text.0 = format!("TR: {}", wheels.top_right.rpm)
            },
            10 => {
                text.0 = format!("BL: {}", wheels.bottom_left.rpm)
            },
            11 => {
                text.0 = format!("BR: {}", wheels.bottom_right.rpm)
            }
            _ => ()
        }
    }
}