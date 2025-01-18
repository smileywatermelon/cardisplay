use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use mevy::{spawn, ui};
use crate::core::states::GameState;
use crate::vehicle::car::Car;
use crate::vehicle::controls::CarActions;
use crate::vehicle::parts::prelude::{Engine, Transmission};
use crate::vehicle::parts::wheels::{Brakes, Wheels};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugState::default());
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States, Reflect)]
pub enum DebugState {
    Enabled,
    #[default]
    Disabled
}

fn toggle_debug(
    state: Res<State<DebugState>>,
    mut next: ResMut<NextState<DebugState>>,
    mut debug: Query<&mut Visibility, With<DebugMenu>>
) {
    let mut debug = debug.single_mut();
    match state.get() {
        DebugState::Enabled => {
            *debug = Visibility::Visible;
            next.set(DebugState::Disabled);
        },
        DebugState::Disabled => {
            *debug = Visibility::Hidden;
            next.set(DebugState::Enabled);
        }
    }
}

#[derive(Component)]
struct DebugMenu;

#[derive(Component)]
struct DebugMarker {
    pub text: String,
    pub id: usize
}

impl DebugMarker {
    pub fn new<S: ToString>(text: S, id: usize) -> (Text, TextFont, DebugMarker){
        (
            Text::from(text.to_string().clone()),
            TextFont::from_font_size(10.0),
            DebugMarker {
                text: text.to_string(),
                id
            },
        )
    }

    fn format<S: ToString>(&self, text: S) -> String {
        format!("{}: {}", self.text, text.to_string())
    }

    fn format_f32(&self, text: f32) -> String {
        self.format(format!("{:.2}", text))
    }
}
