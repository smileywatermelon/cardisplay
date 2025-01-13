use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::Car;
use crate::vehicle::parts::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    #[actionlike[Axis]]
    Throttle,
    Brake,
    GearUp,
    GearDown,
}

pub(crate) fn spawn_car_actions(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_axis(CarActions::Throttle, GamepadAxis::LeftStickY)
        .with(CarActions::Brake, GamepadButton::LeftTrigger2)
        .with(CarActions::GearUp, GamepadButton::DPadUp)
        .with(CarActions::GearDown, GamepadButton::DPadDown);

    commands.spawn(InputManagerBundle::with_map(input_map));
}

pub(crate) fn handle_car_actions(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&Car, &mut Engine, &mut Transmission, &mut Brakes)>
) {
    if let Ok((car, mut engine, mut transmission, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();

        engine.throttle = actions.clamped_value(&CarActions::Throttle);
        engine.rpm = (engine.initial * (engine.throttle * 3.0)) + engine.initial;
        brakes.pressure = actions.pressed(&CarActions::Brake) as i32 as f32;

        if actions.just_pressed(&CarActions::GearUp) {
            transmission.gear_up();
        }
        if actions.just_pressed(&CarActions::GearDown) {
            transmission.gear_down();
        }
    }
}