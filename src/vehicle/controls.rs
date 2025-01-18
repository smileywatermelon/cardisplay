use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::vehicle::car::Car;
use crate::vehicle::parts::prelude::*;

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum CarActions {
    /// `Right Trigger` by default
    #[actionlike(Axis)]
    Throttle,
    /// `Left Trigger` by default
    #[actionlike(Axis)]
    Brake,

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