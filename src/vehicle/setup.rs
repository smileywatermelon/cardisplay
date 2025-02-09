use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;
use crate::vehicle::car::Car;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut singleplayer: ResMut<NextState<SingleplayerState>>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Car::main_car());

    commands.spawn((
        SceneRoot(assets.load("mesh/car.glb")),
        Transform::from_xyz(0.0, 0.0, -5.0),
    ));

    singleplayer.set(SingleplayerState::Finished)
}

pub(crate) fn despawn_car(
    mut commands: Commands,
    car: Query<Entity, With<Car>>
) {
    commands.entity(car.single()).despawn_recursive();
}