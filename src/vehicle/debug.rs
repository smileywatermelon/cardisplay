use bevy::prelude::*;
use crate::vehicle::parts::prelude::*;

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
        });
        parent.spawn(cnode.clone()).with_children(|parent| {
            parent.spawn(Text::new("Transmission"));
            parent.spawn(text("Selected", 2));
        });
        parent.spawn(cnode).with_children(|parent| {
            parent.spawn(Text::new("Wheels"));
            parent.spawn(text("Brakes", 3));
            parent.spawn(text("TL", 4));
            parent.spawn(text("TR", 5));
            parent.spawn(text("BL", 6));
            parent.spawn(text("BR", 7));
        });
    });
}

pub(crate) fn update_car_debug(
    mut car_debug: Query<(&mut Text, &CarDebugMarker)>,
    car: Query<(&Engine, &Transmission, &Wheels, &Brakes)>
) {
    let (engine, transmission, wheels, brakes) = car.single();

    for (mut text, marker) in car_debug.iter_mut() {
        match marker.0 {
            0 => {
                text.0 = format!("RPM: {}", engine.rpm)
            },
            1 => {
                text.0 = format!("THR: {}", engine.throttle)
            },
            2 => {
                text.0 = format!("Gear: {}", transmission.gear_string())
            },
            3 => {
                text.0 = format!("Brakes: {}", brakes.pressure)
            },
            4 => {
                text.0 = format!("TL: {}", wheels.top_left.rpm)
            },
            5 => {
                text.0 = format!("TR: {}", wheels.top_right.rpm)
            },
            6 => {
                text.0 = format!("BL: {}", wheels.bottom_left.rpm)
            },
            7 => {
                text.0 = format!("BR: {}", wheels.bottom_right.rpm)
            },
            _ => ()
        }
    }
}