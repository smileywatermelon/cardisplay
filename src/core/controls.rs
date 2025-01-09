use bevy::prelude::*;
use leafwing_input_manager::{Actionlike, InputManagerBundle};
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::{GamepadStick, InputMap, VirtualDPad, VirtualDPad3D};

pub struct ControlPlugin;
impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(InputManagerPlugin::<PlayerActions>::default())
            .add_systems(Startup, player_controls);
    }
}

#[derive(Actionlike, Clone, Copy, PartialEq, Eq, Hash, Debug, Reflect)]
pub enum PlayerActions {
    #[actionlike(DualAxis)]
    Movement,
    Jump,
}

#[derive(Component)]
pub struct PlayerActionsMarker;

fn player_controls(mut commands: Commands) {
    let input_map = InputMap::default()
        .with_dual_axis(PlayerActions::Movement, VirtualDPad::wasd())
        .with(PlayerActions::Jump, KeyCode::Space)
        .with_dual_axis(PlayerActions::Movement, GamepadStick::LEFT)
        .with(PlayerActions::Jump, GamepadButton::South);

    commands.spawn((
        InputManagerBundle::with_map(input_map),
        Name::new("PlayerActions")
    )).insert(PlayerActionsMarker);

}