use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::inspector_options::std_options::NumberDisplay;

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
#[require(DriveTrain, Brakes)]
pub struct Wheels {
    pub tl: Wheel,
    pub tr: Wheel,
    pub bl: Wheel,
    pub br: Wheel,
}

impl Wheels {
    pub fn new(drivetrain: DriveTrain) -> (Self, DriveTrain, Brakes) {
        let wheels = match drivetrain {
            DriveTrain::FWD => {
                Self {
                    tl: Wheel::powered(),
                    tr: Wheel::powered(),
                    ..Default::default()
                }
            },
            DriveTrain::RWD => {
                Self {
                    bl: Wheel::powered(),
                    br: Wheel::powered(),
                    ..Default::default()
                }
            },
            DriveTrain::AWD => {
                Self {
                    tl: Wheel::powered(),
                    tr: Wheel::powered(),
                    bl: Wheel::powered(),
                    br: Wheel::powered(),
                }
            },
        };
        (wheels, drivetrain, Brakes::default())
    }
}

#[derive(Reflect, InspectorOptions)]
#[reflect(InspectorOptions)]
pub struct Wheel {
    /// Wheel Radius
    #[inspector(min = 0.1, max = 2.0, display = NumberDisplay::Slider)]
    pub radius: f32,
    /// Steering angle
    #[inspector(min = -30.0, max = 30.0, display = NumberDisplay::Slider)]
    pub angle: f32,
    /// Wheel RPM
    #[inspector(min = -10000.0, max = 10000.0, display = NumberDisplay::Slider)]
    pub rpm: f32,
    /// Wheel Grip coefficient
    #[inspector(min = 0.0, max = 1.0, display = NumberDisplay::Slider)]
    pub grip: f32,
    /// Slip ratio
    #[inspector(min = 0.0, max = 1.0, display = NumberDisplay::Slider)]
    pub slip: f32,
    /// Is the wheel powered? (Drivetrain)
    pub powered: bool,
}

impl Default for Wheel {
    fn default() -> Self {
        Self {
            radius: 0.3,
            angle: 0.0,
            rpm: 0.0,
            grip: 0.8,
            slip: 0.0,
            powered: false,
        }
    }
}

impl Wheel {
    pub fn powered() -> Self {
        let mut wheel = Self::default();
        wheel.powered = true;
        wheel
    }
}

#[derive(Component)]
pub struct Brakes {
    pressure: f32,
    power: f32
}

impl Brakes {
    pub fn new(power: f32) -> Self {
        Self {
            power,
            ..default()
        }
    }

    pub fn set_pressure(&mut self, pressure: f32) {
        self.pressure = pressure;
    }

    pub fn pressure(&self) -> f32 {
        self.pressure
    }

    pub fn friction(&self) -> f32 {
        1.0 - (self.power * self.pressure)
    }
}

impl Default for Brakes {
    fn default() -> Self {
        Self {
            pressure: 0.0,
            power: 100.0,
        }
    }
}

#[derive(Component, Default)]
pub enum DriveTrain {
    FWD,
    #[default]
    RWD,
    AWD
}