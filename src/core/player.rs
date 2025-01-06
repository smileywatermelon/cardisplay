use avian3d::prelude::ShapeCaster;
use avian3d::math::{Scalar, Vector, PI};
use avian3d::prelude::{Collider, RigidBody};
use bevy::input::keyboard::KeyboardInput;
use bevy::input::mouse::MouseMotion;
use bevy::math::AspectRatio;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_inspector_egui::egui::CursorGrab;
use bevy_inspector_egui::egui::debug_text::print;
use crate::core::helpers::{text, toggle_grabmode};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(PlayerSettings::default())
            .add_systems(Startup, (
                initialize_grab,
                spawn_player,
                spawn_player_ui,
            ))
            .add_systems(Update, (
                look_player,
                update_player_ui,
                cursor_grab
            ))
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

#[derive(Component)]
#[component(storage = "SparseSet")]
pub struct Grounded;

#[derive(Component)]
pub struct Movement {
    pub acceleration: Scalar,
    pub damping: Scalar,
    pub jump_impulse: Scalar,
    pub slope_angle: Scalar,
    pub gravity: Vector
}

impl Movement {
    pub fn new(acceleration: Scalar, damping: Scalar, jump_impulse: Scalar, slope_angle: Scalar, gravity: Vector) -> Self {
        Self { acceleration, damping, jump_impulse, slope_angle, gravity }
    }
}

impl Default for Movement {
    fn default() -> Self {
        Self::new(30.0, 0.9, 7.0, PI * 0.45, Vector::NEG_Y * 9.81 * 2.0)
    }
}

#[derive(Component, Default)]
#[require(Movement, RigidBody, Collider, ShapeCaster)]
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
        Collider::cylinder(16.0, 0.1),
        Mesh3d(meshes.add(Cylinder::new(16.0, 0.1))),
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
            range: 50.0,
            shadows_enabled: true,
            ..Default::default()
        },
        Transform::from_xyz(0.0, 15.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Player::new(0),
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
            width: Val::Percent(25.0),
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
    });
}

pub fn update_player_ui(
    player: Query<&Transform, With<Player>>,
    camera: Query<&Transform, With<Camera>>,
    mut ui: Query<(&mut Text, &PlayerUi)>,
) {
    if let Ok(player) = player.get_single() {
        if let Ok(camera) = camera.get_single() {
            for (mut text, id) in ui.iter_mut() {
                match id.0 {
                    0 => {
                        let t = player.translation;
                        text.0 = format!("Position: ({:.2}, {:.2}, {:.2})", t.x, t.y, t.z);
                    },
                    1 => {
                        let r = player.rotation;
                        text.0 = format!("Rotation: ({:.2}, {:.2}, {:.2})", r.x, r.y, r.z);
                    },
                    2 => {
                        let d = camera.rotation;
                        text.0 = format!("Rotation: ({:.2}, {:.2}, {:.2})", d.x, d.y, d.z);
                    }
                    _ => ()
                }
            }
        }
    }
}