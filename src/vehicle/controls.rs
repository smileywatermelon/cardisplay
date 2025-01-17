use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::{Car, CarState};
use crate::vehicle::parts::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    /// `Right Trigger` by default
    #[actionlike(Axis)]
    Throttle,
    /// `W` by default
    KThrottle,
    /// `Left Trigger` by default
    #[actionlike(Axis)]
    Brake,
    /// `S` by default
    KBrake,
    /// `West` | `Space` by default, X (Xbox), ▢ (PS)
    HandBrake,
    /// `Right Bumper` | `Arrow Up` by default, RB (Xbox), R1 (PS)
    GearUp,
    /// `Left Bumper` | `Arrow Down` by default, LB (Xbox), L1 (PS)
    GearDown,
    /// `Start` | `Tab` by default, ≡ (Xbox)
    ToggleOn,
    /// `North` | `F` by default, Y (Xbox)
    ExitCar
}

impl CarActions {
    pub fn new() -> InputManagerBundle<Self> {
        let input_map = InputMap::default()
            .with_axis(CarActions::Throttle, GamepadAxis::RightZ)
            .with_axis(CarActions::Brake, GamepadAxis::LeftZ)
            .with(CarActions::HandBrake, GamepadButton::West)
            .with(CarActions::GearUp, GamepadButton::RightTrigger)
            .with(CarActions::GearDown, GamepadButton::LeftTrigger)
            .with(CarActions::ToggleOn, GamepadButton::Start)

            .with(CarActions::KThrottle, KeyCode::KeyW)
            .with(CarActions::KBrake, KeyCode::KeyS)
            .with(CarActions::HandBrake, KeyCode::Space)
            .with(CarActions::GearUp, KeyCode::KeyE)
            .with(CarActions::GearDown, KeyCode::KeyQ)
            .with(CarActions::ToggleOn, KeyCode::KeyF);

        InputManagerBundle::with_map(input_map)
    }
}

#[derive(Resource, Default)]
pub struct CarActionsKeyboard {
    pub throttle: f32,
    pub brake: f32,
}

pub(crate) fn handle_car_actions(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&mut Engine, &mut Transmission, &mut Brakes), With<Car>>,
    mut car_state: ResMut<NextState<CarState>>
) {
    if let Ok((mut engine, mut transmission, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();

        // Start/Stop the car
        if actions.just_pressed(&CarActions::ToggleOn) {
            engine.toggle_on();
        }

        // Transmission
        if actions.just_pressed(&CarActions::GearUp) {
            transmission.gear_up()
        }
        if actions.just_pressed(&CarActions::GearDown) {
            transmission.gear_down()
        }

        if actions.just_pressed(&CarActions::HandBrake) {
            todo!()
        }

        if actions.just_pressed(&CarActions::ExitCar) {
            car_state.set(CarState::OutCar);
            info!("Exited Car")
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum CarPedalMode {
    #[default]
    Keyboard,
    Controller
}

pub(crate) fn handle_gamepad_pedals(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&mut Engine, &mut Transmission, &mut Brakes), With<Car>>,
    time: Res<Time>
) {
    if let Ok((mut engine, mut transmission, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();
        let throttle = actions.clamped_value(&CarActions::Throttle);
        let brake = actions.clamped_value(&CarActions::Brake);

        engine.set_throttle(throttle);
        engine.update_rpm(time.delta_secs());
        brakes.set_pressure(brake);
    }
}

pub(crate) fn handle_kb_pedals(
    actions: Query<&ActionState<CarActions>>,
    mut car: Query<(&mut Engine, &mut Transmission, &mut Brakes), With<Car>>,
    mut kb_actions: ResMut<CarActionsKeyboard>,
    time: Res<Time>
) {
    if let Ok((mut engine, mut transmission, mut brakes)) = car.get_single_mut() {
        let actions = actions.single();

        if actions.pressed(&CarActions::KThrottle) {
            kb_actions.throttle += 0.01;
            info!("KB Throttle: {}", kb_actions.throttle)
        } else {
            kb_actions.throttle -= 0.01;
        }

        if actions.pressed(&CarActions::KBrake) {
            kb_actions.brake += 0.01;
            info!("KB Brake: {}", kb_actions.brake)
        } else {
            kb_actions.brake -= 0.01;
        }

        kb_actions.throttle = kb_actions.throttle.clamp(0.0, 1.0);
        kb_actions.brake = kb_actions.brake.clamp(0.0, 1.0);

        engine.set_throttle(kb_actions.throttle);
        engine.update_rpm(time.delta_secs());
        brakes.set_pressure(kb_actions.brake);
    }
}