use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::inspector_options::std_options::NumberDisplay;

#[derive(Component, Default, Reflect, InspectorOptions)]
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
                    ..default()
                }
            },
            DriveTrain::RWD => {
                Self {
                    bl: Wheel::powered(),
                    br: Wheel::powered(),
                    ..default()
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

fn slip_ratio(rpm: f32, speed: f32) -> f32 {
    let wheel_speed = rpm * 2.0 * PI * 0.3 / 60.0;

    if speed.abs() < 0.1 {
        0.0
    } else {
        (wheel_speed - speed) / speed.abs()
    }
}

impl Wheel {
    pub fn powered() -> Self {
        let mut wheel = Self::default();
        wheel.powered = true;
        wheel
    }

    pub fn calculate_slip_ratio(&mut self, rpm: f32, total_ratio: f32, transform: Transform, linear: LinearVelocity) {
        if !self.powered {
            return
        }
    
        self.rpm = rpm / total_ratio.abs();
        self.slip = slip_ratio(self.rpm, transform.forward().dot(linear.0));
    
        if self.angle != 0.0 {
            Quat::from_rotation_y(self.angle.to_radians()) * Vec3::Z
        } else {
            Vec3::Z
        };
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