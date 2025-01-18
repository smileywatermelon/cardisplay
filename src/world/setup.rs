use avian3d::collision::{Collider, ColliderConstructor, ColliderConstructorHierarchy};
use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Component)]
pub struct WorldComponent;

pub(crate) fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(100.0, 100.0, 1.0),
        Mesh3d(meshes.add(Cuboid::new(100.0, 100.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -10.0, 0.0),
        Name::new("Platform"),
        WorldComponent
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cylinder(1.0, 5.0),
        Mesh3d(meshes.add(Cylinder::new(1.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(232, 120, 56))),
        WorldComponent
    ));

    game_state.set(GameState::SpawnPlayers)
}

pub(crate) fn despawn_world(
    mut commands: Commands,
    objects: Query<Entity, With<WorldComponent>>
) {
    for entity in objects.iter() {
        commands.entity(entity).despawn_recursive();
    }
}