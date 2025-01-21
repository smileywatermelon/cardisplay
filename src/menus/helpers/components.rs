use bevy::prelude::*;
use std::ops::Range;

#[derive(Component)]
pub struct MenuMarker;

#[derive(Component)]
pub struct CycleButton {
    strings: Vec<String>,
    current: usize,
}

impl CycleButton {
    pub fn new(strings: Vec<String>) -> Self {
        Self {
            strings,
            current: 0
        }
    }
    pub fn get(&self) -> String {
        self.strings[self.current].clone()
    }
    pub fn next(&mut self) -> String {
        let length = self.strings.len() - 1;

        self.current = match self.current {
            _ if self.current == length => 0,
            _ => self.current + 1
        };
        self.get()
    }
}

pub trait Ease {
    fn new(range: Range<f32>) -> Self;
    fn add(&mut self, value: f32) -> f32;
    fn sub(&mut self, value: f32) -> f32;
}

pub trait XYEase {
    fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self;
    fn add(&mut self, value: f32) -> (f32, f32);
    fn add_xy(&mut self, x: f32, y: f32) -> (f32, f32);
    fn sub(&mut self, value: f32) -> (f32, f32);
    fn sub_xy(&mut self, x: f32, y: f32) -> (f32, f32);
}

macro_rules! ease {
    ($name:ident) => {
        #[derive(Component)]
        pub struct $name {
            value: f32,
            range: Range<f32>,
            pub ease: bool
        }

        impl Ease for $name {
            fn new(range: Range<f32>) -> Self {
                Self {
                    value: range.start, range,
                    ease: false
                }
            }

            fn add(&mut self, value: f32) -> f32 {
                self.value += value;
                self.value = self.value.min(self.range.end);

                self.value
            }

            fn sub(&mut self, value: f32) -> f32 {
                self.value -= value;
                self.value = self.value.max(self.range.start);

                self.value
            }
        }
    };
}

macro_rules! xy_ease {
    ($name:ident) => {
        #[derive(Component)]
        pub struct $name {
            /// current x scale
            x: f32,
            /// min and max x scales
            x_range: Range<f32>,
            /// current y scale
            y: f32,
            /// min and max y scales
            y_range: Range<f32>,
            /// ease up or down, represented by true or false
            pub ease: bool
        }

        impl XYEase for $name {
            fn new(x_range: Range<f32>, y_range: Range<f32>) -> Self {
                Self {
                    x: x_range.start, x_range,
                    y: y_range.start, y_range,
                    ease: false
                }
            }

            fn add(&mut self, value: f32) -> (f32, f32) {
                self.x += value;
                self.x = self.x.min(self.x_range.end);

                self.y += value;
                self.y = self.y.min(self.y_range.end);

                (self.x, self.y)
            }

            fn add_xy(&mut self, x: f32, y: f32) -> (f32, f32) {
                self.x += x;
                self.x = self.x.min(self.x_range.end);

                self.y += y;
                self.y = self.y.min(self.y_range.end);

                (self.x, self.y)
            }

            fn sub(&mut self, value: f32) -> (f32, f32) {
                self.x -= value;
                self.x = self.x.max(self.x_range.start);

                self.y -= value;
                self.y = self.y.max(self.y_range.start);

                (self.x, self.y)
            }

            fn sub_xy(&mut self, x: f32, y: f32) -> (f32, f32) {
                self.x -= x;
                self.x = self.x.max(self.x_range.start);

                self.y -= y;
                self.y = self.y.max(self.y_range.start);

                (self.x, self.y)
            }
        }
    };
}

ease!(UiFadeEase);
ease!(UiScaleEase);
xy_ease!(UiPositionEase);