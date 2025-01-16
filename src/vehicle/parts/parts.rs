use bevy::prelude::*;

#[derive(Component)]
#[require(EngineSetup)] 
pub struct Engine {
    // Engine RPMS
    /// Current Engine RPM
    rpm: f32,
    /// Engine idle RPM
    initial: f32,
    /// Engine red line RPM
    redline: f32,
    /// Engine Throttle
    throttle: f32,
    // Engine (De)Acceleration
    /// Acceleration
    accel_rate: f32,
    /// Deceleration
    decel_rate: f32,
    /// Is the Engine on?
    on: bool
}

impl Engine {
    pub fn new(initial: f32, redline: f32, accel_rate: f32, decel_rate: f32) -> Self {
        Self {
            rpm: initial.clone(),
            throttle: 0.0,
            initial,
            redline,
            accel_rate,
            decel_rate,
            on: false
        }
    }

    pub fn set_throttle(&mut self, throttle: f32) {
        if self.on {
            self.throttle = throttle;
        }
    }

    pub fn toggle_on(&mut self) {
        self.on = !self.on;
    }

    pub fn update_rpm(&mut self, time: f32) {
        if self.on {
            let target_rpm = self.initial + (self.throttle * (self.redline - self.initial));

            let rpm_delta = if self.rpm < target_rpm {
                self.accel_rate * time
            } else {
                self.decel_rate * time
            };

            if self.rpm < target_rpm {
                self.rpm = (self.rpm + rpm_delta).min(target_rpm);
            } else {
                self.rpm = (self.rpm - rpm_delta).max(target_rpm);
            }
        } else {
            self.rpm = 0.0
        }
    }

    // Getters
    pub fn rpm(&self) -> f32 { self.rpm }
    pub fn initial(&self) -> f32 { self.initial }
    pub fn redline(&self) -> f32 { self.redline }
    pub fn throttle(&self) -> f32 { self.throttle }
    pub fn accel_rate(&self) -> f32 { self.accel_rate }
    pub fn decel_rate(&self) -> f32 { self.decel_rate }
    pub fn is_on(&self) -> bool { self.on }
}

impl Default for Engine {
    /// EK9 Engine Specs
    fn default() -> Self {
        Self {
            rpm: 0.0,
            initial: 950.0,
            redline: 8700.0,

            throttle: 0.0,

            accel_rate: 6000.0,
            decel_rate: 4000.0,

            on: false
        }
    }
}

#[derive(Component, Default)]
pub enum EngineSetup {
    /// Inline 4 Engine
    #[default]
    I4,
    /// Inline 6 Engine
    I6,
    /// V6 Engine
    V6,
    /// V8 Engine
    V8
}

#[derive(Component)]
pub struct Transmission {
    ratios: Vec<f32>,
    reverse: f32,
    final_drive: f32,
    /// Describes the selected gear
    ///
    /// 1 - N are normal gears
    ///
    /// 0 is neutral
    ///
    /// -1 is Reverse
    selected: i32
}

impl Transmission {
    pub fn gear(&self) -> i32 {
        self.selected
    }

    pub fn gear_string(&self) -> String {
        match self.gear() {
            -1 => {
                "R".to_string()
            },
            0 => {
                "N".to_string()
            },
            g => {
                g.to_string()
            }
        }
    }

    pub fn ratio(&self) -> f32 {
        match self.selected {
            -1 => -self.reverse,
            0 => 0.0,
            _ => self.ratios[self.selected as usize - 1]
        }
    }

    pub fn ratios(&self) -> Vec<f32> {
        self.ratios.clone()
    }

    pub fn final_drive(&self) -> f32 {
        self.final_drive
    }

    pub fn gear_up(&mut self) {
        if !(self.selected + 1 > self.ratios.len() as i32) {
            self.selected += 1;
        }
    }

    pub fn gear_down(&mut self) {
        if !(self.selected == -1) {
            self.selected -= 1
        }
    }

    pub fn turn_off(&mut self) {
        self.selected = 0;
    }

    /// Returns `-1.0` if true, `1.0` if false
    ///
    /// Used for `Wheels` calculation
    pub fn is_reverse(&self) -> f32 {
        if self.selected == -1 {
            return -1.0;
        }
        1.0
    }
}

impl Default for Transmission {
    /// Honda Civic EK9 Gear Ratios
    fn default() -> Self {
        Self {
            ratios: vec![3.230, 2.105, 1.458, 1.107, 0.848],
            reverse: 3.000,
            final_drive: 4.400,
            // Neutral
            selected: 0
        }
    }
}