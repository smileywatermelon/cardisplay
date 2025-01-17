use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use mevy::{spawn, ui};
use crate::core::states::GameState;
use crate::vehicle::car::Car;
use crate::vehicle::controls::{CarActions, CarActionsKeyboard};
use crate::vehicle::parts::parts::{Engine, Transmission};
use crate::vehicle::parts::wheels::{Brakes, Wheels};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.insert_state(DebugState::default())
            .add_systems(Startup, spawn_debug_menu)
            .add_systems(Update, (
                toggle_debug.run_if(input_just_pressed(KeyCode::Escape)),
                update_debug_menu.run_if(in_state(DebugState::Enabled))
            ).run_if(in_state(GameState::Running)))
            ;
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
    pub fn new<S: ToString>(text: S, id: usize) -> (Text, DebugMarker){
        (
            Text::from(text.to_string().clone()),
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

fn spawn_debug_menu(mut commands: Commands) {
    commands.spawn((ui!((
        display: flex;
        flex_direction: column;
        size: 20%, 50%;
    )), DebugMenu, Name::new("DebugMenu"), Visibility::Hidden)).with_children(|parent| {
        parent.spawn(Text::new("Debug Menu"));

        parent.spawn(Text::new("Engine"));
        parent.spawn(DebugMarker::new("Throttle", 0));
        parent.spawn(DebugMarker::new("GP Throttle", 1));
        parent.spawn(DebugMarker::new("KB Throttle", 2));
        parent.spawn(DebugMarker::new("RPM", 3));
        parent.spawn(DebugMarker::new("On/Off", 4));
        parent.spawn(Text::new("Transmission"));
        parent.spawn(DebugMarker::new("Gear", 5));
        parent.spawn(DebugMarker::new("Ratio", 6));
        parent.spawn(Text::new("Wheels"));
        parent.spawn(DebugMarker::new("Brakes", 7));
        parent.spawn(DebugMarker::new("GP Brakes", 8));
        parent.spawn(DebugMarker::new("KB Brakes", 9));
        parent.spawn(DebugMarker::new("RPM", 10));
        parent.spawn(Text::new("Actions"));
        parent.spawn(DebugMarker::new("GP Throttle", 11));
        parent.spawn(DebugMarker::new("GP Brake", 12));
        parent.spawn(DebugMarker::new("KB Throttle", 13));
        parent.spawn(DebugMarker::new("KB Brake", 14));
    });
}

fn update_debug_menu(
    car: Query<(&Engine, &Transmission, &Wheels, &Brakes, &ActionState<CarActions>), With<Car>>,

    kb_actions: Res<CarActionsKeyboard>,
    mut debug: Query<(&mut Text, &DebugMarker)>
) {
    if let Ok((engine, transmission, wheels, brakes, actions)) = car.get_single() {
        for (mut text, debug) in debug.iter_mut() {
            match debug.id {
                0 => {
                    text.0 = debug.format_f32(engine.throttle())
                },
                1 => {
                    text.0 = debug.format_f32(0.0)
                },
                2 => {
                    text.0 = debug.format_f32(kb_actions.throttle)
                },
                3 => {
                    text.0 = debug.format_f32(engine.rpm())
                },
                4 => {
                    text.0 = debug.format(engine.is_on())
                }
                5 => {
                    text.0 = debug.format(transmission.gear_string())
                },
                6 => {
                    text.0 = debug.format(transmission.ratio())
                },
                7 => {
                    text.0 = debug.format(brakes.pressure())
                },
                8 => {
                    text.0 = debug.format_f32(0.0)
                },
                9 => {
                    text.0 = debug.format_f32(kb_actions.brake)
                },
                10 => {
                    text.0 = debug.format(wheels.top_left.rpm)
                },
                11 => {
                    text.0 = debug.format(actions.clamped_value(&CarActions::Throttle))
                },
                12 => {
                    text.0 = debug.format(actions.clamped_value(&CarActions::Brake))
                },
                13 => {
                    text.0 = debug.format(actions.pressed(&CarActions::KThrottle))
                },
                14 => {
                    text.0 = debug.format(actions.pressed(&CarActions::KBrake))
                }
                _ => ()
            }
        }
    }
}