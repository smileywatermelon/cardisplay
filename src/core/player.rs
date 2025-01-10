use avian3d::prelude::*;
use avian3d::math::{AdjustPrecision, Quaternion, Scalar, Vector, PI};
use bevy::input::common_conditions::input_just_pressed;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_inspector_egui::prelude::*;
use leafwing_input_manager::prelude::*;
use crate::core::controls::{PlayerActions, PlayerActionsMarker};
use crate::core::helpers::{text, toggle_grabmode};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerSettings::default())
            .insert_resource(DampingFactorResource(0.9))
            .init_resource::<DampingFactorResource>()
            .register_type::<DampingFactorResource>()
            .add_systems(Startup, (
                initialize_grab,
                spawn_player,
                spawn_player_ui,
            ))
            .add_systems(Update, (
                look_player,
                update_player_ui,
                cursor_grab,
                restart_player.run_if(input_just_pressed(KeyCode::KeyR))
            ))
            .add_systems(Update, (
                update_grounded,
                apply_gravity,
                move_player,
                apply_movement_damping
                ).chain()
            )
            .add_systems(
                PostProcessCollisions,
                kinematic_controller_collisions,
            )
        ;
    }
}

#[derive(Resource)]
pub struct PlayerSettings {
    pub sensitivity: f32,
    pub speed: f32,
    pub zoom: f32,
    pub acc: f32
}

impl Default for PlayerSettings {
    fn default() -> Self {
        Self {
            sensitivity: 100.0,
            speed: 10.0,
            zoom: 0.05,
            acc: 10000.0
        }
    }
}

/// A marker component indicating that an entity is using a character controller.
#[derive(Component)]
pub struct CharacterController;

/// A marker component indicating that an entity is on the ground.
#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

/// A bundle that contains the components needed for a basic
/// kinematic character controller.
#[derive(Bundle)]
pub struct CharacterControllerBundle {
    character_controller: CharacterController,
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
}

/// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}

impl MovementBundle {
    pub const fn new(
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        Self {
            acceleration: MovementAcceleration(acceleration),
            damping: MovementDampingFactor(damping),
            jump_impulse: JumpImpulse(jump_impulse),
            max_slope_angle: MaxSlopeAngle(max_slope_angle),
        }
    }
}

impl Default for MovementBundle {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45)
    }
}

impl CharacterControllerBundle {
    pub fn new(collider: Collider, gravity: Vector) -> Self {
        // Create shape caster as a slightly smaller version of collider
        let mut caster_shape = collider.clone();
        caster_shape.set_scale(Vector::ONE * 0.99, 10);

        Self {
            character_controller: CharacterController,
            rigid_body: RigidBody::Kinematic,
            collider,
            ground_caster: ShapeCaster::new(
                caster_shape,
                Vector::ZERO,
                Quaternion::default(),
                Dir3::NEG_Y,
            )
                .with_max_distance(0.2),
            gravity: ControllerGravity(gravity),
            movement: MovementBundle::default(),
        }
    }

    pub fn with_movement(
        mut self,
        acceleration: Scalar,
        damping: Scalar,
        jump_impulse: Scalar,
        max_slope_angle: Scalar,
    ) -> Self {
        self.movement = MovementBundle::new(acceleration, damping, jump_impulse, max_slope_angle);
        self
    }
}

#[derive(Component, Default)]
pub struct Player {
    pub id: usize
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self { id }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        RigidBody::Static,
        Collider::cylinder(100.0, 1.0),
        Mesh3d(meshes.add(Cylinder::new(100.0, 1.0))),
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

    commands.spawn((
        Player::new(0),
        CharacterControllerBundle::new(Collider::capsule(0.4, 1.0), Vector::NEG_Y * 9.81 * 2.0)
            .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
        Name::new("Player")
    ))
        .with_child(Camera3d::default());
}

pub fn initialize_grab(
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        toggle_grabmode(&mut window);
    }
}

pub fn cursor_grab(
    keys: Res<ButtonInput<KeyCode>>,
    mut primary_window: Query<&mut Window, With<PrimaryWindow>>
) {
    if let Ok(mut window) = primary_window.get_single_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grabmode(&mut window);
        }
    } else {
        warn!("Primary window not found for `cursor_grab`!");
    }
}

// Looking
pub fn look_player(
    mut camera: Query<&mut Transform, With<Camera>>,
    settings: Res<PlayerSettings>,
    primary_window: Query<&Window>,
    mut mouse: EventReader<MouseMotion>,
    time: Res<Time>
) {
    for window in primary_window.iter() {
        for mut transform in camera.iter_mut() {
            for ev in mouse.read() {
                let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
                match window.cursor_options.grab_mode {
                    CursorGrabMode::None => (),
                    _ => {
                        let window_scale = window.height().min(window.width());
                        pitch -= ((settings.sensitivity / settings.acc) * ev.delta.y * window_scale).to_radians() * time.delta_secs();
                        yaw -= ((settings.sensitivity / settings.acc) * ev.delta.x * window_scale).to_radians() * time.delta_secs();
                    }
                }

                pitch = pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
            }
        }
    }
}

const SPEED: f32 = 100.0;

fn move_player(
    time: Res<Time>,
    mut player: Query<(
        &MovementAcceleration,
        &JumpImpulse,
        &mut LinearVelocity,
        Has<Grounded>
    )>,
    camera: Query<&Transform, With<Camera>>,
    actions: Query<&ActionState<PlayerActions>, With<PlayerActionsMarker>>
) {
    let delta = time.delta_secs_f64().adjust_precision();

    if let Ok((movement, jump, mut linear, grounded)) = player.get_single_mut() {
        let actions = actions.single();
        let movement_input = actions.clamped_axis_pair(&PlayerActions::Movement);
        let camera = camera.single();

        let forward = camera.forward() * movement_input.y * delta * SPEED;
        let right = camera.right() * movement_input.x * delta * SPEED;
        let right = right.normalize_or_zero();

        linear.x += forward.x + right.x;
        linear.z += forward.z + right.z;

        if actions.just_pressed(&PlayerActions::Jump) {
            if grounded {
                linear.y = jump.0;
            }
        }
    }
}

fn apply_gravity(
    time: Res<Time>,
    mut player: Query<(&ControllerGravity, &mut LinearVelocity)>
) {
    let delta = time.delta_secs_f64().adjust_precision();

    if let Ok((gravity, mut linear)) = player.get_single_mut() {
        linear.0 += gravity.0 * delta;
    }
}

#[derive(Reflect, Resource, Clone, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct DampingFactorResource(
    #[inspector(min = 0.0, max = 1.0)]
    pub Scalar
);

impl Default for DampingFactorResource {
    fn default() -> Self {
        Self(0.9)
    }
}

fn set_damping(
    mut player: Query<&mut MovementDampingFactor, With<Player>>,
    damp_resource: Res<DampingFactorResource>
) {
    if let Ok(mut damping_factor) = player.get_single_mut() {
        damping_factor.0 = damp_resource.0;
    }
}

fn apply_movement_damping(
    mut player: Query<(&MovementDampingFactor, &mut LinearVelocity)>
) {
    if let Ok((damping_factor, mut linear)) = player.get_single_mut() {
        linear.x *= damping_factor.0;
        linear.z *= damping_factor.0;
    }
}

fn update_grounded(
    mut commands: Commands,
    mut query: Query<
        (Entity, &ShapeHits, &Rotation, Option<&MaxSlopeAngle>),
        With<CharacterController>
    >
) {
    for (entity, hits, rotation, max_slope_angle) in &mut query {
        let is_grounded = hits.iter().any(|hit| {
            if let Some(angle) = max_slope_angle {
                (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= angle.0
            } else {
                true
            }
        });

        if is_grounded {
            commands.entity(entity).insert(Grounded);
        } else {
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

#[allow(clippy::type_complexity)]
fn kinematic_controller_collisions(
    collisions: Res<Collisions>,
    bodies: Query<&RigidBody>,
    collider_parents: Query<&ColliderParent, Without<Sensor>>,
    mut character_controllers: Query<
        (
            &mut Position,
            &Rotation,
            &mut LinearVelocity,
            Option<&MaxSlopeAngle>,
        ),
        (With<RigidBody>, With<CharacterController>),
    >,
    time: Res<Time>,
) {
    // Iterate through collisions and move the kinematic body to resolve penetration
    for contacts in collisions.iter() {
        // Get the rigid body entities of the colliders (colliders could be children)
        let Ok([collider_parent1, collider_parent2]) =
            collider_parents.get_many([contacts.entity1, contacts.entity2])
        else {
            continue;
        };

        // Get the body of the character controller and whether it is the first
        // or second entity in the collision.
        let is_first: bool;

        let character_rb: RigidBody;
        let is_other_dynamic: bool;

        let (mut position, rotation, mut linear_velocity, max_slope_angle) =
            if let Ok(character) = character_controllers.get_mut(collider_parent1.get()) {
                is_first = true;
                character_rb = *bodies.get(collider_parent1.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent2.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else if let Ok(character) = character_controllers.get_mut(collider_parent2.get()) {
                is_first = false;
                character_rb = *bodies.get(collider_parent2.get()).unwrap();
                is_other_dynamic = bodies
                    .get(collider_parent1.get())
                    .is_ok_and(|rb| rb.is_dynamic());
                character
            } else {
                continue;
            };

        // This system only handles collision response for kinematic character controllers.
        if !character_rb.is_kinematic() {
            continue;
        }

        // Iterate through contact manifolds and their contacts.
        // Each contact in a single manifold shares the same contact normal.
        for manifold in contacts.manifolds.iter() {
            let normal = if is_first {
                -manifold.global_normal1(rotation)
            } else {
                -manifold.global_normal2(rotation)
            };

            let mut deepest_penetration: Scalar = Scalar::MIN;

            // Solve each penetrating contact in the manifold.
            for contact in manifold.contacts.iter() {
                if contact.penetration > 0.0 {
                    position.0 += normal * contact.penetration;
                }
                deepest_penetration = deepest_penetration.max(contact.penetration);
            }

            // For now, this system only handles velocity corrections for collisions against static geometry.
            if is_other_dynamic {
                continue;
            }

            // Determine if the slope is climbable or if it's too steep to walk on.
            let slope_angle = normal.angle_between(Vector::Y);
            let climbable = max_slope_angle.is_some_and(|angle| slope_angle.abs() <= angle.0);

            if deepest_penetration > 0.0 {
                // If the slope is climbable, snap the velocity so that the character
                // up and down the surface smoothly.
                if climbable {
                    // Points in the normal's direction in the XZ plane.
                    let normal_direction_xz =
                        normal.reject_from_normalized(Vector::Y).normalize_or_zero();

                    // The movement speed along the direction above.
                    let linear_velocity_xz = linear_velocity.dot(normal_direction_xz);

                    // Snap the Y speed based on the speed at which the character is moving
                    // up or down the slope, and how steep the slope is.
                    //
                    // A 2D visualization of the slope, the contact normal, and the velocity components:
                    //
                    //             ╱
                    //     normal ╱
                    // *         ╱
                    // │   *    ╱   velocity_x
                    // │       * - - - - - -
                    // │           *       | velocity_y
                    // │               *   |
                    // *───────────────────*

                    let max_y_speed = -linear_velocity_xz * slope_angle.tan();
                    linear_velocity.y = linear_velocity.y.max(max_y_speed);
                } else {
                    // The character is intersecting an unclimbable object, like a wall.
                    // We want the character to slide along the surface, similarly to
                    // a collide-and-slide algorithm.

                    // Don't apply an impulse if the character is moving away from the surface.
                    if linear_velocity.dot(normal) > 0.0 {
                        continue;
                    }

                    // Slide along the surface, rejecting the velocity along the contact normal.
                    let impulse = linear_velocity.reject_from_normalized(normal);
                    linear_velocity.0 = impulse;
                }
            } else {
                // The character is not yet intersecting the other object,
                // but the narrow phase detected a speculative collision.
                //
                // We need to push back the part of the velocity
                // that would cause penetration within the next frame.

                let normal_speed = linear_velocity.dot(normal);

                // Don't apply an impulse if the character is moving away from the surface.
                if normal_speed > 0.0 {
                    continue;
                }

                // Compute the impulse to apply.
                let impulse_magnitude =
                    normal_speed - (deepest_penetration / time.delta_secs_f64().adjust_precision());
                let mut impulse = impulse_magnitude * normal;

                // Apply the impulse differently depending on the slope angle.
                if climbable {
                    // Avoid sliding down slopes.
                    linear_velocity.y -= impulse.y.min(0.0);
                } else {
                    // Avoid climbing up walls.
                    impulse.y = impulse.y.max(0.0);
                    linear_velocity.0 -= impulse;
                }
            }
        }
    }
}

#[derive(Debug, Component)]
pub struct PlayerUi(pub usize);

pub fn spawn_player_ui(
    mut commands: Commands,
) {
    commands.spawn((
        Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            align_content: AlignContent::Center,
            justify_content: JustifyContent::Center,
            margin: UiRect::all(Val::Percent(0.5)),
            padding: UiRect::all(Val::Percent(0.5)),
            width: Val::Percent(30.0),
            aspect_ratio: Some(3.0),
            ..Default::default()
        },
        BackgroundColor(Color::srgb_u8(30, 31, 34)),
        BorderRadius::all(Val::Percent(15.0)),
    )).with_children(|parent| {
        parent.spawn((
            text("Position: ()", 16.0, None),
            PlayerUi(0)
        ));
        parent.spawn((
            text("Rotation: ()", 16.0, None),
            PlayerUi(1)
        ));
        parent.spawn((
            text("C-Rotation: ()", 16.0, None),
            PlayerUi(2)
        ));
        parent.spawn((
            text("Velocity: ()", 16.0, None),
            PlayerUi(3)
        ));
    });
}

pub fn update_player_ui(
    player: Query<(&Transform, &LinearVelocity), With<Player>>,
    camera: Query<&Transform, With<Camera>>,
    mut ui: Query<(&mut Text, &PlayerUi)>,
) {
    if let Ok((transform, linear)) = player.get_single() {
        if let Ok(camera) = camera.get_single() {
            for (mut text, id) in ui.iter_mut() {
                match id.0 {
                    0 => {
                        let t = transform.translation;
                        text.0 = format!("Position: ({:.2}, {:.2}, {:.2})", t.x, t.y, t.z);
                    },
                    1 => {
                        let r = transform.rotation;
                        text.0 = format!("Rotation: ({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
                    },
                    2 => {
                        let d = camera.rotation;
                        text.0 = format!("Rotation: ({:.2}, {:.2}, {:.2})", d.x, d.y, d.z);
                    },
                    3 => {
                        let v = linear;
                        text.0 = format!("Velocity: ({:.2}, {:.2}, {:.2})", v.x, v.y, v.z);
                    }
                    _ => ()
                }
            }
        }
    }
}

pub fn restart_player(
    mut player: Query<(&mut Transform, &mut LinearVelocity), With<Player>>
) {
    if let Ok((mut transform, mut linear)) = player.get_single_mut() {
        transform.translation = Vec3::new(0.0, 10.0, 0.0);
        linear.x = 0.0;
        linear.z = 0.0;
    }
}