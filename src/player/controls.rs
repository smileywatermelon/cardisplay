use avian3d::prelude::LinearVelocity;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use leafwing_input_manager::prelude::*;
use crate::player::player::{Player, PlayerSettings};

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum PlayerActions {
    #[actionlike(DualAxis)]
    Look,
    #[actionlike(DualAxis)]
    Move,
    Jump,
    EnterCar,
    // Debug
    DebugResetPlayer
}

impl PlayerActions {
    pub fn new(player: Player) -> InputMap<Self> {
        let input_map = InputMap::default()
            // KBM
            .with_dual_axis(PlayerActions::Look, MouseMove::default())
            .with_dual_axis(PlayerActions::Move, VirtualDPad::wasd())
            .with(PlayerActions::Jump, KeyCode::Space)
            .with(PlayerActions::EnterCar, KeyCode::KeyE)
            .with(PlayerActions::DebugResetPlayer, KeyCode::KeyR)
            // Gamepad
            .with_dual_axis(PlayerActions::Look, GamepadStick::RIGHT.with_deadzone_symmetric(0.1))
            .with_dual_axis(PlayerActions::Move, GamepadStick::LEFT.with_deadzone_symmetric(0.1))
            .with(PlayerActions::Jump, GamepadButton::South)
            .with(PlayerActions::EnterCar, GamepadButton::North)
            ;

        input_map
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayerSetup {
    #[default]
    None,
    Singleplayer,
    Multiplayer
}

/// How much to divide the player sensitivity by
const SENSITIVITY_FACTOR: f32 = 10000.0;

pub(crate) fn handle_player_actions(
    mut camera: Query<&mut Transform, With<Camera>>,
    primary_window: Query<&Window, With<PrimaryWindow>>,
    mut player: Query<(&ActionState<PlayerActions>, &mut LinearVelocity), With<Player>>,
    settings: Query<&PlayerSettings, With<Player>>,
    time: Res<Time>
) {
    for window in primary_window.iter() {
        let settings = settings.single();
        let (actions, mut linear) = player.single_mut();

        if let Ok(mut camera) = camera.get_single_mut() {
            // -- Looking
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

            // -- End Looking

            let forward = camera.forward();
            let right = camera.right().normalize_or_zero();

            linear.x += forward.x + right.x;
            linear.z += forward.z + right.z;

            // -- Movement

            // -- End Movement



            // -- Other

            // -- End Other
        }
    }
}