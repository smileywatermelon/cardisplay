use bevy::prelude::*;

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