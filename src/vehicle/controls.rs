use avian3d::prelude::{AngularVelocity, LinearVelocity};
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::MainCar;
use crate::vehicle::parts::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    /// `Right Trigger` by default
    #[actionlike(Axis)]
    Throttle,
    /// `Left Trigger` by default
    #[actionlike(Axis)]
    Brake,
    /// `Right Joystick` by default
    #[actionlike(Axis)]
    Turn,

    /// `W` by default
    KThrottle,
    /// `S` by default
    KBrake,
    /// `A` by default
    KTurnLeft,
    /// `D` by default
    KTurnRight,

    /// `West` | `Space` by default
    HandBrake,

    /// `Right Bumper` | `Arrow Up` by default
    GearUp,
    /// `Left Bumper` | `Arrow Down` by default
    GearDown,

    /// `Start` | `F` by default
    ToggleOn,
}

impl CarActions {
    pub fn new() -> InputManagerBundle<Self> {
        let input_map = InputMap::default()
            .with_axis(CarActions::Throttle, GamepadAxis::RightZ)
            .with_axis(CarActions::Brake, GamepadAxis::LeftZ)
            .with_axis(CarActions::Turn, GamepadAxis::RightStickX)
            .with(CarActions::HandBrake, GamepadButton::West)
            .with(CarActions::GearUp, GamepadButton::RightTrigger)
            .with(CarActions::GearDown, GamepadButton::LeftTrigger)
            .with(CarActions::ToggleOn, GamepadButton::Start)

            .with(CarActions::KThrottle, KeyCode::KeyW)
            .with(CarActions::KBrake, KeyCode::KeyS)
            .with(CarActions::KTurnLeft, KeyCode::KeyA)
            .with(CarActions::KTurnRight, KeyCode::KeyD)
            .with(CarActions::HandBrake, KeyCode::Space)
            .with(CarActions::GearUp, KeyCode::KeyE)
            .with(CarActions::GearDown, KeyCode::KeyQ)
            .with(CarActions::ToggleOn, KeyCode::KeyF);

        InputManagerBundle::with_map(input_map)
    }
}

pub fn handle_camera(
    car: Query<&Transform, With<MainCar>>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<MainCar>)>,
) {
    if let Ok(car) = car.get_single() {
        let mut camera = camera.single_mut();

        let offset = Vec3::new(0.0, 5.0, -10.0);
        let rotated_offset = car.rotation.mul_vec3(offset);

        camera.translation = car.translation + rotated_offset;
        camera.look_at(car.translation, Vec3::Y);
    }
}

pub fn handle_controls(
    mut car: Query<(
        &mut Engine,
        &mut Transmission,
        &mut Wheels,
        &DriveTrain,
        &mut LinearVelocity,
        &mut AngularVelocity,
    ), With<MainCar>>,
    controls: Query<&ActionState<CarActions>>
) {
    let (mut engine, mut transmission, mut wheels, drivetrain, mut linear, mut angular) = car.single_mut();
    let prev_ratio = transmission.ratio();

    let controls = controls.single();

    // Button Controls

    if controls.just_pressed(&CarActions::GearUp) {
        transmission.gear_up();

        let ratio = transmission.ratio();
        engine.shift_rpm(prev_ratio, ratio);

        info!("↑ {} - {}", transmission.ratio(), engine.rpm())
    } else if controls.just_pressed(&CarActions::GearDown) {
        transmission.gear_down();

        let ratio = transmission.ratio();
        engine.shift_rpm(prev_ratio, ratio);

        info!("↓ {} - {}", transmission.ratio(), engine.rpm())
    }

    // Axes Controls

    // Pedals
    let throttle = controls.clamped_value(&CarActions::Throttle);
    let kthrottle = controls.pressed(&CarActions::KThrottle) as i32 as f32;
    let brake = controls.clamped_value(&CarActions::Brake);

    engine.set_throttle(kthrottle);

    // Steering

    let turn = controls.clamped_value(&CarActions::Turn);
    let kturn = (controls.pressed(&CarActions::KTurnRight) as i32 - controls.pressed(&CarActions::KTurnLeft) as i32) as f32;

    wheels.tl.angle = kturn * 30.0;
    wheels.tr.angle = kturn * 30.0;
}