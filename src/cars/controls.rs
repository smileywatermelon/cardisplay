use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::cars::car::Car;
use crate::cars::parts::{Engine, Transmission};
use crate::cars::wheels::Brakes;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    Throttle,
    Brake,
    GearUp,
    GearDown,
}

pub(crate) fn spawn_car_actions(mut commands: Commands) {
    let input_map = InputMap::default()
        .with(CarActions::Throttle, GamepadButton::RightTrigger2)
        .with(CarActions::Brake, GamepadButton::LeftTrigger2)
        .with(CarActions::GearUp, GamepadButton::DPadUp)
        .with(CarActions::GearDown, GamepadButton::DPadDown);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub(crate) fn handle_car_actions(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&Car, &mut Engine, &mut Transmission, &mut Brakes)>
) {
    if let Ok((car, mut engine, mut trans, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();

        engine.throttle = actions.pressed(&CarActions::Throttle) as i32 as f32;
        brakes.pressure = actions.pressed(&CarActions::Brake) as i32 as f32;

        if actions.just_pressed(&CarActions::GearUp) {
            trans.gear_up();
        }
        if actions.just_pressed(&CarActions::GearDown) {
            trans.gear_down();
        }
    }
}