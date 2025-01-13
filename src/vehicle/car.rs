use bevy::prelude::*;
use crate::core::helpers::prelude::*;
use crate::core::states::GameState;
use super::parts::prelude::*;

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
        Engine::new(1500.0),
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb_u8(242, 181, 27),
            perceptual_roughness: 1.0,
            ..default()
        })),
        Transform::from_xyz(10.0, 10.0, 15.0)
    ));


}

pub(crate) fn spawn_engine_audio(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn(AudioMarker::from_const(AudioConst::ENGINE_AUDIO, assets));

    game_state.set(GameState::Running);
}

/// What to divide `Engine.rpm` by for audio speed
const RPM_AUDIO_FACTOR: f32 = 1000.0;

pub(crate) fn update_engine_noise(
    car: Query<&Engine, With<Car>>,
    mut audio: Query<(&mut AudioSink, &AudioId)>
) {
    for (mut audio_sink, id) in audio.iter_mut() {
        if id.0 == AudioConst::ENGINE_AUDIO.id {
            let engine = car.single();
            let audio_factor = (engine.rpm / RPM_AUDIO_FACTOR).abs() ;

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
        let mut rpm = (engine.rpm / transmission.ratio()) * engine.throttle * brakes.friction() * transmission.is_reverse();

        if rpm.is_nan() || rpm.is_infinite() {
            rpm = 0.0;
        }

        wheels.all_rpm(rpm);
    }
}