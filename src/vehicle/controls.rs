use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::Car;
use crate::vehicle::parts::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    #[actionlike(Axis)]
    Throttle,
    KThrottle,
    #[actionlike(Axis)]
    Brake,
    KBrake,
    GearUp,
    GearDown,
}

#[derive(Resource, Default)]
pub struct CarActionsKeyboard {
    pub throttle: f32,
    pub brake: f32,
}

pub(crate) fn spawn_car_actions(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_axis(CarActions::Throttle, GamepadAxis::RightZ)
        .with_axis(CarActions::Brake, GamepadAxis::LeftZ)
        .with(CarActions::GearUp, GamepadButton::DPadUp)
        .with(CarActions::GearDown, GamepadButton::DPadDown)
        
        .with(CarActions::KThrottle, KeyCode::KeyW)
        .with(CarActions::KBrake, KeyCode::KeyS)
        .with(CarActions::GearUp, KeyCode::KeyE)
        .with(CarActions::GearDown, KeyCode::KeyQ)
        ;

    println!("Actions Spawned");
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
        brakes.pressure = actions.clamped_value(&CarActions::Brake);

        println!("{}", actions.clamped_value(&CarActions::Throttle));
        println!("{}", actions.clamped_value(&CarActions::Brake));

        if actions.just_pressed(&CarActions::GearUp) {
            transmission.gear_up();
        }
        if actions.just_pressed(&CarActions::GearDown) {
            transmission.gear_down();
        }
    }
}

pub(crate) fn handle_kb_car_actions(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&Car, &mut Engine, &mut Transmission, &mut Brakes)>,
    mut kb_actions: ResMut<CarActionsKeyboard>
) {
    if let Ok((car, mut engine, mut transmission, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();

        if actions.pressed(&CarActions::KThrottle) {
            kb_actions.throttle += 0.01;
        } else {
            kb_actions.throttle -= 0.01;
        }

        if actions.pressed(&CarActions::KBrake) {
            kb_actions.brake += 0.01;
        } else {
            kb_actions.brake -= 0.01;
        }

        kb_actions.throttle = kb_actions.throttle.clamp(0.0, 1.0);
        kb_actions.brake = kb_actions.brake.clamp(0.0, 1.0);

        engine.throttle = kb_actions.throttle;
        engine.rpm = (engine.initial * (engine.throttle * 3.0)) + engine.initial;
        brakes.pressure = kb_actions.brake;
    }
}