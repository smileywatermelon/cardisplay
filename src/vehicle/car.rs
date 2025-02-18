use bevy::prelude::*;
use crate::core::states::GameState;
use super::parts::prelude::*;

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

impl Car {
    pub fn main_car() -> (Self, MainCar) {
        (
            Self(0), MainCar
            )
    }
}

/// Specifies the client car
#[derive(Component)]
pub struct MainCar;