use bevy::prelude::*;
use crate::core::helpers::prelude::*;
use crate::core::states::GameState;
use crate::vehicle::controls::CarActions;
use super::parts::prelude::*;

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

#[derive(Component)]
pub struct CurrentCar;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut game_state: ResMut<NextState<GameState>>
) {
    commands.spawn((
        Car(0),
        CurrentCar,
        ));

    game_state.set(GameState::Running)
}

pub(crate) fn despawn_car(
    mut commands: Commands,
    car: Query<Entity, With<Car>>
) {
    commands.entity(car.single()).despawn_recursive();
}