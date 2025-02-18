use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::MainCar;

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

        let offset = Vec3::new(0.0, 5.0, 10.0);
        let rotated_offset = car.rotation.mul_vec3(offset);

        camera.translation = car.translation + rotated_offset;
        camera.look_at(car.translation, Vec3::Y);
        println!("Hello")
    }
    println!("Outside")
}

pub fn handle_axes() {

}