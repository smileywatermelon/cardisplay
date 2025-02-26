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
    let torque = engine.throttle() * engine.rpm() * 0.1;

    let mut force = Vec3::ZERO;
    let rpm = engine.rpm();

    wheels.tl.calculate_slip_ratio(rpm, total_ratio, *transform, *linear);

    linear.0 += transform.rotation * force * time.delta_secs();

    let brake_force = brakes.friction() * brakes.pressure() * 1000.0;
    linear.0 *= 1.0 - brake_force * time.delta_secs();
}