pub mod wheels;
pub mod engine;
pub mod transmission;

pub mod prelude {
    pub use super::engine::{Engine, EngineSetup};
    pub use super::transmission::Transmission;
    pub use super::wheels::{Brakes, DriveTrain, Wheel, Wheels};
}