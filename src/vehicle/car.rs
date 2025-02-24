use avian3d::math::PI;
use avian3d::prelude::LinearVelocity;
use bevy::prelude::*;
use crate::core::states::GameState;
use super::parts::prelude::*;

#[derive(Component)]
#[require(Engine, Transmission, Wheels)]
pub struct Car(pub usize);

impl Car {
    pub fn main_car() -> (Self, (Wheels, DriveTrain, Brakes), MainCar) {
        (Self(0), Wheels::new(DriveTrain::FWD), MainCar)
    }
}

/// Specifies the client car
#[derive(Component)]
pub struct MainCar;

fn slip_ratio(rpm: f32, speed: f32) -> f32 {
    let wheel_speed = rpm * 2.0 * PI * 0.3 / 60.0;

    if speed.abs() < 0.1 {
        0.0
    } else {
        (wheel_speed - speed) / speed.abs()
    }
}

fn wheel_calculation(mut wheel: &mut Wheel, rpm: f32, total_ratio: f32, transform: Transform, linear: LinearVelocity) -> &mut Wheel {
    if !wheel.powered {
        return &mut wheel
    }

    wheel.rpm = rpm / total_ratio.abs();
    wheel.slip = slip_ratio(wheel.rpm, transform.forward().dot(linear.0));

    let direction = if wheel.angle != 0.0 {
        Quat::from_rotation_y(wheel.angle.to_radians()) * Vec3::Z
    } else {
        Vec3::Z
    };

    &mut wheel
}

pub fn handle_car(
    mut car: Query<(
        &mut Engine,
        &Transmission,
        &mut Wheels,
        &mut LinearVelocity,
        &Transform,
        &Brakes,
        &DriveTrain
    ), With<MainCar>>,

    time: Res<Time>
) {
    let (mut engine, transmission, mut wheels, mut linear, transform, brakes, drivetrain) = car.single_mut();

    engine.update_rpm(time.delta_secs());

    let total_ratio = transmission.ratio() * transmission.final_drive();

    // Probably need to add a torque curve
    let engine_torque = engine.throttle() * engine.rpm() * 0.1;

    let mut force = Vec3::ZERO;

    for wheel in wheels.wheels {
        if wheel
        wheel.rpm = engine.rpm() / total_ratio.abs();
        wheel.slip = slip_ratio(wheel.rpm, transform.forward().dot(linear.0));

        let direction = if wheel.angle != 0.0 {
            Quat::from_rotation_y(wheel.angle.to_radians()) * Vec3::Z
        } else {
            Vec3::Z
        };
    }

    linear.0 += transform.rotation * force * time.delta_secs();

    let brake_force = brakes.friction() * brakes.pressure() * 1000.0;
    linear.0 *= 1.0 - brake_force * time.delta_secs();
}