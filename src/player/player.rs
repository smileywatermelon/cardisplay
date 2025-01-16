use avian3d::math::{Quaternion, Scalar, Vector, PI};
use avian3d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::core::states::GameState;
use crate::player::controls::PlayerActions;

#[derive(Component, Clone, Copy, Debug)]
#[require(PlayerSettings)]
pub struct Player;

impl Player {
    pub fn new() -> (Self, InputMap<PlayerActions>) {
        let player = Self;
        (
            player,
            PlayerActions::new(player),
        )
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum PlayerSetup {
    #[default]
    Single,
    Multiplayer,
}

pub(crate) fn spawn_players(
    mut commands: Commands,
    gamepad: Query<(Entity, &Gamepad)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    commands.spawn((
        Player::new(),
        PlayerControllerBundle::new(Collider::cylinder(1.0, 4.0)),
        Name::new("Player")
    ));

    game_state.set(GameState::SpawnVehicles)
}

#[derive(Component)]
pub struct PlayerSettings {
    /// Should be `0.0` - `100.0`
    ///
    /// Default: `75.0`
    pub sensitivity: f32,
    /// Movement speed multiplier
    ///
    /// Default: `10.0`
    pub speed: f32,
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 75.0,
            speed: 10.0,
        }
    }
}

// Copied directly from avian3d example, maybe update but makes sense

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);
#[derive(Component)]
#[component(storage = "SparseSet")]
pub(crate) struct Grounded;

#[derive(Bundle)]
pub(crate) struct PlayerMovementBundle {
    pub acceleration: MovementAcceleration,
    pub damping: MovementDampingFactor,
    pub jump_impulse: JumpImpulse,
    pub max_slope_angle: MaxSlopeAngle,
}

impl PlayerMovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for PlayerMovementBundle {
    /// Values copied from avian3d example
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

#[derive(Component)]
pub struct PlayerController;

#[derive(Bundle)]
pub(crate) struct PlayerControllerBundle {
    pub player_controller: PlayerController,
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub ground_caster: ShapeCaster,
    pub locked_axes: LockedAxes,
    pub movement: PlayerMovementBundle
}

impl PlayerControllerBundle {
    pub fn new(collider: Collider) -> Self {
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            player_controller: PlayerController,
            rigid_body: RigidBody::Dynamic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y
            )
                .with_max_distance(0.2),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            movement: PlayerMovementBundle::default()
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar
    ) -> Self {
        self.movement = PlayerMovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}