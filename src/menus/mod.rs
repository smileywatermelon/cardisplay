mod mainmenu;
mod states;
mod settings;

use crate::menus::states::MainMenu;
use bevy::prelude::*;
use crate::core::states::GameState;
use crate::menus::mainmenu::init_menu;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MainMenu>()
            .add_systems(OnEnter(GameState::MainMenu), init_menu);
    }
}