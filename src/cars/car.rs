use std::ops::Add;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance};
use crate::cars::parts::{Engine, EngineSetup, Transmission};
use crate::cars::wheels::{Brakes, Wheels};

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

#[derive(Component)]
pub struct TireMarker;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Car(0),
        Engine { rpm: 100.0, throttle: 0.0 }
    ));

    commands.spawn((
        TireMarker,
        Mesh3d(meshes.add(Cylinder::new(2.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, 1.0, 0.0),
    ));
}

#[derive(Resource)]
pub(crate) struct EngineHandle(Handle<AudioInstance>);

pub(crate) fn spawn_engine_audio(
    mut commands: Commands,
    assets: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let handle = audio.play(assets.load("audio/engine.mp3"))
        .looped()
        .handle();

    commands.insert_resource(EngineHandle(handle));
}

pub(crate) fn update_engine_noise(
    car: Query<&Engine, With<Car>>,
    engine_audio: Res<EngineHandle>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    let engine = car.single();

    if let Some(audio) = audio_instances.get_mut(&engine_audio.0) {
        audio.set_playback_rate(engine.rpm as f64, Default::default());
    }
}

pub(crate) fn update_wheel_rpm(
    mut car: Query<(&Engine, &Transmission, &mut Wheels, &Brakes)>
) {
    if let Ok((mut engine, transmission, mut wheels, brakes)) = car.get_single_mut() {
        //               Convert Engine RPM to Wheel RPM with Gear Ratio
        //               |                                  Then multiply by throttle
        //               |                                  |                  Then multiply by brake factor
        //               |                                  |                  |
        let mut rpm = ((engine.rpm / transmission.ratio()) * engine.throttle) * brakes.friction();

        if rpm.is_nan() || rpm.is_infinite() {
            rpm = 0.0;
        }

        wheels.all_rpm(rpm);
    }
}