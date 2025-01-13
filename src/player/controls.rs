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
    pub fn single(player: Player) -> InputMap<Self> {
        let input_map = InputMap::default()
            // KBM Controls
            .with(Self::DebugResetPlayer, KeyCode::KeyR)
            // Controller
            .with_dual_axis(Self::Look, GamepadStick::RIGHT)
            .with_dual_axis(Self::Move, GamepadStick::LEFT)
            .with(Self::Jump, GamepadButton::South)
            .with(Self::EnterCar, GamepadButton::North)
            .with_gamepad(player.gamepad);

        input_map
    }
    pub fn multiplayer(players: Vec<Player>) -> Vec<InputMap<Self>> {
        let mut input_maps: Vec<InputMap<Self>> = Vec::with_capacity(players.len());

        for player in players {
            let input_map = Self::single(player);
            input_maps.push(input_map);
        }

        input_maps
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayerSetup {
    #[default]
    None,
    Singleplayer,
    Multiplayer
}

pub(crate) fn spawn_players(
    mut commands: Commands,

) {

}

pub(crate) fn player_actions(mut commands: Commands) {
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

    commands.spawn((
        InputManagerBundle::with_map(input_map),
        Name::new("PlayerActions")
    ));
}

/// How much to divide the player sensitivity by
const SENSITIVITY_FACTOR: f32 = 10000.0;

pub(crate) fn old_look_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    settings: Query<&PlayerSettings, With<Player>>,
    primary_window: Query<&Window>,
    time: Res<Time>,
    mut mouse: EventReader<MouseMotion>
) {
    for window in primary_window.iter() {
        for mut transform in camera.iter_mut() {
            for ev in mouse.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);

                let settings = settings.single();

                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= ((settings.sensitivity / SENSITIVITY_FACTOR) * ev.delta.y * window_scale).to_radians() * time.delta_secs();
                        yaw   -= ((settings.sensitivity / SENSITIVITY_FACTOR) * ev.delta.x * window_scale).to_radians() * time.delta_secs();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

pub(crate) fn player_look(
    mut camera: Query<&mut Transform, With<Camera>>,
    actions: Query<&ActionState<PlayerActions>, With<Player>>,
    primary_window: Query<&PrimaryWindow>
) {
    if let Ok(window) = primary_window.get_single() {
        for mut transform in camera.iter_mut() {

        }
    }
}