use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

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
    let mut input_map = InputMap::default();

    input_map.insert_dual_axis(PlayerActions::Movement, VirtualDPad::wasd());
    input_map.insert(PlayerActions::Jump, KeyCode::Space);

    input_map.insert_dual_axis(PlayerActions::Movement, GamepadStick::LEFT);
    input_map.insert(PlayerActions::Jump, GamepadButton::South);

    commands.spawn((
        InputManagerBundle::with_map(input_map),
        Name::new("PlayerActions")
    )).insert(PlayerActionsMarker);

}