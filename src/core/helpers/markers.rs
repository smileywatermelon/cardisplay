use bevy::prelude::*;

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

    pub fn from_const(audio_const: AudioConst, assets: Res<AssetServer>) -> Self {
        Self::new(assets.load(audio_const.path), audio_const.settings, audio_const.id)
    }
}

// Soon gonna create a program that generates the constants code automatically,
// by reading the file names and writing to a file. If project isn't that complex
// I can just do it by hand

pub struct AudioConst {
    pub path: &'static str,
    pub settings: PlaybackSettings,
    pub id: usize,
}

impl AudioConst {
    pub fn new(path: &'static str, settings: PlaybackSettings, id: usize) -> Self {
        Self { path, settings, id }
    }

    pub fn new_loop(path: &'static str, id: usize) -> Self {
        Self::new(path, PlaybackSettings::LOOP, id)
    }

    pub const fn new_const(path: &'static str, settings: PlaybackSettings, id: usize) -> Self {
        Self { path, settings, id }
    }

    pub const fn new_const_loop(path: &'static str, id: usize) -> Self {
        Self::new_const(path, PlaybackSettings::LOOP, id)
    }

    // Defined Audio Constants

    pub const ENGINE_AUDIO: Self = Self::new_const_loop("audio/engine.ogg", 0);
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