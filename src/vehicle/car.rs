use bevy::prelude::*;
use crate::core::helpers::prelude::*;
use crate::core::states::GameState;
use super::parts::prelude::*;

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum CarState {
    #[default]
    OutCar,
    InCar
}

#[derive(Component)]
pub struct TireMarker;

pub(crate) fn spawn_car(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut game_state: ResMut<NextState<GameState>>
) {
    commands.spawn((
        Car(0),
        Engine::default(),
        Mesh3d(meshes.add(Cuboid::from_length(10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(242, 181, 27),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(10.0, 10.0, 15.0),
        Name::new("Car")
    ));

    game_state.set(GameState::Running);
}

pub(crate) fn spawn_engine_audio(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.spawn(AudioMarker::from_const(AudioConst::ENGINE_IDLE, &assets));
    commands.spawn(AudioMarker::from_const(AudioConst::ENGINE_DRIVE_1, &assets));
    commands.spawn(AudioMarker::from_const(AudioConst::ENGINE_DRIVE_2, &assets));
    commands.spawn(AudioMarker::from_const(AudioConst::ENGINE_DRIVE_3, &assets));
}

pub(crate) fn update_engine_noise(
    car: Query<&Engine, With<Car>>,
    mut audio: Query<(&AudioSink, &AudioId)>
) {
    for (audio_sink, id) in audio.iter_mut() {
        if id.0 == AudioConst::ENGINE_IDLE.id {
            let engine = car.single();
            let audio_factor = (engine.rpm() / engine.initial()).abs();

            if audio_factor != 0.0 {
                if audio_sink.is_paused() {
                    audio_sink.play()
                }
                audio_sink.set_speed(audio_factor);
            } else {
                audio_sink.pause()
            }
        }
    }
}

pub(crate) fn update_wheel_rpm(
    mut car: Query<(&Engine, &Transmission, &mut Wheels, &Brakes)>
) {
    if let Ok((mut engine, transmission, mut wheels, brakes)) = car.get_single_mut() {
        // (Engine RPM / Ratio) * Throttle * Brake Friction * (-1.0 | 1.0)
        let ratio = transmission.ratio() * transmission.final_drive();
        let reverse = transmission.is_reverse();

        let mut rpm = ((engine.rpm() / (ratio)) * reverse) * brakes.friction();

        if rpm.is_nan() || rpm.is_infinite() {
            rpm = 0.0;
        }

        wheels.all_rpm(rpm);
    }
}