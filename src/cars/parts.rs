use bevy::prelude::*;

#[derive(Component, Default)]
#[require(EngineSetup)]
pub struct Engine {
    pub rpm: f32,
    pub throttle: f32,
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
}

impl Default for Transmission {
    fn default() -> Self {
        /// Honda Civic EK9 Gear Ratios
        Self {
            ratios: vec![3.230, 2.105, 1.458, 1.107, 0.848],
            reverse: 3.000,
            // Neutral
            selected: 0
        }
    }
}