use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use leafwing_input_manager::prelude::*;
use crate::core::helpers::prelude::set_grabmode;
use crate::player::physics::{Grounded, JumpImpulse};
use crate::player::player::{MainPlayer, PlayerSettings};
use crate::player::states::{ClientState, PlayerCarState};

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum PlayerActions {
    #[actionlike(DualAxis)]
    Look,
    #[actionlike(DualAxis)]
    Move,
    Jump,
    ToggleCar,
    TogglePause,
    // Debug
    DebugResetPlayer
}

impl PlayerActions {
    pub fn new() -> InputManagerBundle<Self> {
        let input_map = InputMap::default()
            // KBM
            .with_dual_axis(PlayerActions::Look, MouseMove::default())
            .with_dual_axis(PlayerActions::Move, VirtualDPad::wasd())
            .with(PlayerActions::Jump, KeyCode::Space)
            .with(PlayerActions::ToggleCar, KeyCode::KeyE)
            .with(PlayerActions::TogglePause, KeyCode::Escape)
            .with(PlayerActions::DebugResetPlayer, KeyCode::KeyR)
            // Gamepad
            .with_dual_axis(PlayerActions::Look, GamepadStick::RIGHT.with_deadzone_symmetric(0.1))
            .with_dual_axis(PlayerActions::Move, GamepadStick::LEFT.with_deadzone_symmetric(0.1))
            .with(PlayerActions::Jump, GamepadButton::South)
            .with(PlayerActions::ToggleCar, GamepadButton::North)
            .with(PlayerActions::TogglePause, GamepadButton::Start)
            ;

        InputManagerBundle::with_map(input_map)
    }
}

/// How much to divide the player sensitivity by
const SENSITIVITY_FACTOR: f32 = 10000.0;

pub(crate) fn handle_player_look(
    mut camera: Query<&mut Transform, With<Camera>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    player: Query<(&ActionState<PlayerActions>, &PlayerSettings)>,
    time: Res<Time>
) {
    for window in primary_window.iter() {
        let (actions, settings) = player.single();

        if let Ok(mut camera) = camera.get_single_mut() {
            let (mut yaw, mut pitch, _) = camera.rotation.to_euler(EulerRot::YXZ);

            let look_delta = actions.clamped_axis_pair(&PlayerActions::Look);

            match window.cursor_options.grab_mode {
                CursorGrabMode::None => (),
                _ => {
                    let window_scale = window.height().min(window.width());
                    pitch -= ((settings.sensitivity / SENSITIVITY_FACTOR) * look_delta.y * window_scale).to_radians() * time.delta_secs();
                    yaw   -= ((settings.sensitivity / SENSITIVITY_FACTOR) * look_delta.x * window_scale).to_radians() * time.delta_secs();
                }
            }

            pitch = pitch.clamp(-1.54, 1.54);

            camera.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            info!("{:?}", camera.rotation);
        }
    }
}

pub(crate) fn enable_look(
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    set_grabmode(&mut window.single_mut(), true);
}

pub(crate) fn disable_look(
    mut window: Query<&mut Window, With<PrimaryWindow>>
) {
    set_grabmode(&mut window.single_mut(), false);
}

pub(crate) fn handle_player_move(
    mut player: Query<(
        &ActionState<PlayerActions>,
        &PlayerSettings,
        &mut LinearVelocity,
        &JumpImpulse,
        Has<Grounded>
    ), With<MainPlayer>>,
    camera: Query<&Transform, With<Camera>>,
    time: Res<Time>
) {
    if let Ok((actions, settings, mut linear, jump, grounded)) = player.get_single_mut() {
        let camera = camera.single();
        let delta = time.delta_secs();
        let movement = actions.clamped_axis_pair(&PlayerActions::Move);

        let forward = camera.forward() * movement.y * delta * settings.speed;
        let mut right = camera.right() * movement.x * delta * settings.speed;
        right = right.normalize_or_zero();

        linear.x += forward.x + right.x;
        linear.z += forward.z + right.z;

        if actions.just_pressed(&PlayerActions::Jump) {
            if grounded {
                linear.y = jump.0;
            }
        }
    }
}

pub(crate) fn handle_player_actions(
    mut player: Query<(&ActionState<PlayerActions>, &mut Transform, &mut LinearVelocity)>,
    car_state: Res<State<PlayerCarState>>,
    mut car_next_state: ResMut<NextState<PlayerCarState>>,
) {
    let (actions, mut transform, mut linear) = player.single_mut();

    if actions.just_pressed(&PlayerActions::DebugResetPlayer) && car_state.get().clone() != PlayerCarState::InCar {
        transform.translation.x = 0.0;
        transform.translation.y = 0.0;
        transform.translation.z = 0.0;

        linear.0 = Vec3::ZERO;

        info!("Reset Player")
    }

    if actions.just_pressed(&PlayerActions::ToggleCar) {
        info!("Toggled Car");
        match car_state.get() {
            PlayerCarState::InCar => {
                car_next_state.set(PlayerCarState::OutCar);
                info!("Exited Car");
            },
            PlayerCarState::OutCar => {
                car_next_state.set(PlayerCarState::InCar);
                info!("Entered Car");
            }
        }
    }
}

pub(crate) fn toggle_pause(
    player: Query<&ActionState<PlayerActions>>,
    client_state: Res<State<ClientState>>,
    mut client_next_state: ResMut<NextState<ClientState>>,
) {
    let actions = player.single();

    if actions.just_pressed(&PlayerActions::TogglePause) {
        match client_state.get() {
            ClientState::Paused => {
                client_next_state.set(ClientState::Running);
                info!("Client Running");
            },
            ClientState::Running => {
                client_next_state.set(ClientState::Paused);
                info!("Client Paused");
            }
        }
    }
}