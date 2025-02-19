use avian3d::prelude::{Collider, Friction, RigidBody};
use bevy::prelude::*;
use crate::core::states::GameState;
use crate::server::states::SingleplayerState;
use crate::vehicle::car::Car;
use crate::vehicle::controls::CarActions;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut singleplayer: ResMut<NextState<SingleplayerState>>,
    assets: Res<AssetServer>,
) {
    commands.spawn((
        Car::main_car(),
        SceneRoot(assets.load(
            GltfAssetLabel::Scene(0).from_asset("mesh/car.glb")
        )),
        Transform::from_xyz(0.0, 5.0, -5.0),
        RigidBody::Dynamic,
        Collider::cuboid(
            1.430 * 2.0,
            0.408 * 2.0,
            2.470 * 2.0
        ),
        Friction::new(0.5)
    ));

    commands.spawn(CarActions::new());

    singleplayer.set(SingleplayerState::Finished)
}

pub(crate) fn despawn_car(
    mut commands: Commands,
    car: Query<Entity, With<Car>>
) {
    commands.entity(car.single()).despawn_recursive();
}