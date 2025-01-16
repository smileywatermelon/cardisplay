use bevy::{audio::Volume, prelude::*};

#[derive(Component)]
pub struct AudioId(pub usize);

#[derive(Bundle)]
pub struct AudioMarker {
    pub audio: AudioPlayer,
    pub settings: PlaybackSettings,
    pub id: AudioId,
}

impl AudioMarker {
    pub fn new(audio: Handle<AudioSource>, settings: PlaybackSettings, id: usize) -> Self {
        AudioMarker { audio: AudioPlayer::new(audio), settings, id: AudioId(id) }
    }

    pub fn from_const(audio_const: AudioConst, assets: &Res<AssetServer>) -> Self {
        Self::new(
            assets.load(audio_const.path), 
            PlaybackSettings {
                mode: match audio_const.audio_loop {
                    true => bevy::audio::PlaybackMode::Loop,
                    false => bevy::audio::PlaybackMode::Once
                },
                volume: Volume::new(audio_const.volume),
                speed: audio_const.speed,
                paused: audio_const.paused,
                ..default()
            }, 
            audio_const.id
        )
    }
}

// Soon gonna create a program that generates the constants code automatically,
// by reading the file names and writing to a file. If project isn't that complex
// I can just do it by hand

pub struct AudioConst {
    pub path: &'static str,
    pub volume: f32,
    pub speed: f32,
    pub paused: bool,
    pub audio_loop: bool,
    pub id: usize,
}

impl AudioConst {
    pub const fn new(path: &'static str, volume: f32, speed: f32, paused: bool, audio_loop: bool, id: usize) -> Self {
        Self { path, volume, speed, paused, audio_loop, id }
    }

    pub const fn new_loop(path: &'static str, id: usize) -> Self {
        Self::new(path, 1.0, 1.0, false, true, id)
    }

    pub const fn new_loop_paused(path: &'static str, id: usize) -> Self {
        Self::new(path, 1.0, 1.0, true, true, id)
    }

    pub const fn new_loop_volume(path: &'static str, volume: f32, id: usize) -> Self {
        Self::new(path, volume, 1.0, false, true, id)
    }

    pub const fn new_loop_speed(path: &'static str, speed: f32, id: usize) -> Self {
        Self::new(path, 1.0, speed, false, true, id)
    }

    // Defined Audio Constants

    pub const ENGINE_IDLE: Self = Self::new_loop_paused("audio/engine/idle.ogg", 0);
    pub const ENGINE_DRIVE_1: Self = Self::new_loop_paused("audio/engine/drive-1.ogg", 1);
    pub const ENGINE_DRIVE_2: Self = Self::new_loop_paused("audio/engine/drive-2.ogg", 2);
    pub const ENGINE_DRIVE_3: Self = Self::new_loop_paused("audio/engine/drive-3.ogg", 3);

}

#[derive(Component)]
pub struct TextId(pub usize);

#[derive(Bundle)]
pub struct TextMarker {
    pub text: Text,
    pub text_font: TextFont,
    pub id: TextId,
}

impl TextMarker {
    pub fn new<S: ToString>(text: S, text_font: TextFont, id: usize) -> Self {
        Self {
            text: Text::new(text.to_string()),
            text_font,
            id: TextId(id),
        }
    }

    pub fn from_text<S: ToString>(text: S, id: usize) -> Self {
        Self::new(text, TextFont::default(), id)
    }

    pub fn from_text_size<S: ToString>(text: S, size: f32, id: usize) -> Self {
        Self::new(text, TextFont::from_font_size(size), id)
    }

    pub fn from_text_font<S: ToString>(text: S, font: Handle<Font>, id: usize) -> Self {
        Self::new(text, TextFont::from_font(font), id)
    }

    // Defined Text Ids
}