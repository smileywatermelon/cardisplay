use avian3d::collision::Collider;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::player::controls::PlayerActions;
use crate::player::physics::PlayerControllerBundle;
use crate::player::player::{MainPlayer, PlayerSettings};
use crate::server::states::SingleplayerState;

pub(crate) fn spawn_player(
    mut commands: Commands,
    mut singleplayer: ResMut<NextState<SingleplayerState>>,
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    commands.spawn((
        MainPlayer,
        PlayerActions::new(),
        PlayerSettings::default(),
        PlayerControllerBundle::new(Collider::cylinder(1.0, 4.0)),
        Name::new("Player")
    ));

    info!("Player Spawned");

    singleplayer.set(SingleplayerState::SpawnVehicles)
}

pub(crate) fn despawn_player(
    mut commands: Commands,
    player: Query<Entity, With<MainPlayer>>
) {
    commands.entity(player.single()).despawn_recursive();
}