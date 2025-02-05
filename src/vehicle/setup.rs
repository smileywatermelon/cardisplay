use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;
use crate::vehicle::car::Car;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut singleplayer: ResMut<NextState<SingleplayerState>>,
) {
    commands.spawn(Car::main_car());

    singleplayer.set(SingleplayerState::Finished)
}

pub(crate) fn despawn_car(
    mut commands: Commands,
    car: Query<Entity, With<Car>>
) {
    commands.entity(car.single()).despawn_recursive();
}