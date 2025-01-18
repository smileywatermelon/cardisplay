use bevy::prelude::*;
use serde::Deserialize;

#[derive(Component)]
#[require(EngineSetup)]
pub struct Engine {
    // Engine RPMS
    /// Current Engine RPM
    rpm: f32,
    /// Engine idle RPM
    initial: f32,
    /// Engine red line RPM
    redline: f32,
    /// Engine Throttle
    throttle: f32,
    // Engine (De)Acceleration
    /// Acceleration
    accel_rate: f32,
    /// Deceleration
    decel_rate: f32,
    /// Is the Engine on?
    on: bool,
    /// Engine Sound ID
    sound_id: usize
}

impl Engine {
    pub fn new(initial: f32, redline: f32, accel_rate: f32, decel_rate: f32, sound_id: usize) -> Self {
        Self {
            rpm: initial.clone(),
            throttle: 0.0,
            initial,
            redline,
            accel_rate,
            decel_rate,
            on: false,
            sound_id
        }
    }

    pub fn set_throttle(&mut self, throttle: f32) {
        if self.on {
            self.throttle = throttle;
        }
    }

    pub fn toggle_on(&mut self) {
        self.on = !self.on;
    }

    pub fn update_rpm(&mut self, time: f32) {
        if self.on {
            let target_rpm = self.initial + (self.throttle * (self.redline - self.initial));

            let rpm_delta = if self.rpm < target_rpm {
                self.accel_rate * time
            } else {
                self.decel_rate * time
            };

            if self.rpm < target_rpm {
                self.rpm = (self.rpm + rpm_delta).min(target_rpm);
            } else {
                self.rpm = (self.rpm - rpm_delta).max(target_rpm);
            }
        } else {
            self.rpm = 0.0
        }
    }

    // Getters
    pub fn rpm(&self) -> f32 { self.rpm }
    pub fn initial(&self) -> f32 { self.initial }
    pub fn redline(&self) -> f32 { self.redline }
    pub fn throttle(&self) -> f32 { self.throttle }
    pub fn accel_rate(&self) -> f32 { self.accel_rate }
    pub fn decel_rate(&self) -> f32 { self.decel_rate }
    pub fn is_on(&self) -> bool { self.on }
    pub fn sound_id(&self) -> usize { self.sound_id }
}

impl Default for Engine {
    /// EK9 Engine Specs
    fn default() -> Self {
        Self {
            rpm: 0.0,
            initial: 950.0,
            redline: 8700.0,

            throttle: 0.0,

            accel_rate: 6000.0,
            decel_rate: 4000.0,

            on: false,
            sound_id: 0
        }
    }
}

#[derive(Component, Default)]
pub enum EngineSetup {
    /// Inline 4 Engine
    #[default]
    I4,
    /// Inline 6 Engine
    I6,
    /// V6 Engine
    V6,
    /// V8 Engine
    V8
}

pub struct EngineHandle {
    pub id: usize,
    pub idle: Handle<AudioSource>,
    pub low: Handle<AudioSource>,
    pub mid: Handle<AudioSource>,
    pub high: Handle<AudioSource>,
}

impl EngineHandle {
    pub fn from_id(engine_id: usize, assets: &Res<AssetServer>) -> Self {
        let path = "audio/engine/engine_";

        Self {
            id: engine_id,
            idle: assets.load(format!("{}{}/idle.ogg", path, engine_id)),
            low:  assets.load(format!("{}{}/low.ogg", path, engine_id)),
            mid:  assets.load(format!("{}{}/mid.ogg", path, engine_id)),
            high: assets.load(format!("{}{}/high.ogg", path, engine_id)),
        }
    }
}

#[derive(Resource)]
pub struct EngineSounds {
    pub sounds: Vec<EngineHandle>
}

impl EngineSounds {
    pub fn get_handle(&self, id: usize) -> Option<&EngineHandle> {
        self.sounds.get(id)
    }
    pub fn get_mut_handle(&mut self, id: usize) -> Option<&mut EngineHandle> {
        self.sounds.get_mut(id)
    }
}

#[derive(Deserialize, Asset, TypePath)]
pub struct EngineFile {
    pub engine_ids: Vec<usize>
}

pub(crate) fn spawn_engine_sounds(
    mut commands: Commands,
    assets: Res<AssetServer>
) {
    let handle = assets.load("audio/engine/engines.json");

}