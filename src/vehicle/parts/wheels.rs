use bevy::prelude::*;

#[derive(Component, Default)]
#[require(DriveTrain, Brakes)]
pub struct Wheels {
    pub top_left: Wheel,
    pub top_right: Wheel,
    pub bottom_left: Wheel,
    pub bottom_right: Wheel
}

impl Wheels {
    pub fn all_rpm(&mut self, rpm: f32) {
        self.top_left.rpm = rpm;
        self.top_right.rpm = rpm;
        self.bottom_left.rpm = rpm;
        self.bottom_right.rpm = rpm;
    }
}

pub struct Wheel {
    pub rpm: f32,
    pub grip: f32,
    pub angle: f32
}

impl Default for Wheel {
    fn default() -> Self {
        Self {
            rpm: 0.0,
            grip: 0.5,
            angle: 0.0
        }
    }
}

#[derive(Component)]
pub struct Brakes {
    pub pressure: f32,
    power: f32
}

impl Brakes {
    pub fn new(power: f32) -> Self {
        Self {
            power,
            ..default()
        }
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