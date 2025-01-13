pub mod parts;
pub mod wheels;

pub mod prelude {
    pub use super::parts::{Engine, EngineSetup, Transmission};
    pub use super::wheels::{Brakes, DriveTrain, Wheel, Wheels};
}