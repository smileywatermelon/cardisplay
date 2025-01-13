use avian3d::collision::{Collider, ColliderConstructor, ColliderConstructorHierarchy};
use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use crate::core::states::GameState;

pub(crate) fn spawn_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    assets: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        RigidBody::Static,
        ColliderConstructorHierarchy::new(ColliderConstructor::ConvexDecompositionFromMesh),
        SceneRoot(assets.load("mesh/track.glb#Scene0")),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0),
    ));

    commands.spawn((
        RigidBody::Dynamic,
        Collider::cylinder(1.0, 5.0),
        Mesh3d(meshes.add(Cylinder::new(1.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(232, 120, 56)))
    ));

    commands.spawn((
        PointLight {
            intensity: 2_000_000.0,
            range: 5000.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    game_state.set(GameState::SpawnPlayers)
}