use bevy::prelude::*;
use crate::core::helpers::prelude::*;
use crate::core::states::GameState;
use crate::vehicle::controls::CarActions;
use super::parts::prelude::*;

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

#[derive(Component)]
pub struct MainCar;

#[derive(Bundle, Default)]
pub struct CarBundle {
    pub engine: Engine,
    pub tranmission: Transmission,
    pub drive_train: DriveTrain,
    pub wheels: Wheels,
    pub brakes: Brakes,
}

impl CarBundle {
    pub fn main_car(self) -> (Self, MainCar) {
        (self, MainCar)
    }
}

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>
) {
    commands.spawn(CarBundle::default().main_car());

    game_state.set(GameState::Running)
}

pub(crate) fn despawn_car(
    mut commands: Commands,
    car: Query<Entity, With<Car>>
) {
    commands.entity(car.single()).despawn_recursive();
}