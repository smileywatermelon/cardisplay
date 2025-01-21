use bevy::prelude::*;
use crate::core::states::GameState;

#[derive(Resource, Clone)]
pub struct GlobalFont(pub Handle<Font>);

impl GlobalFont {
    pub fn handle(&self) -> Handle<Font> { self.0.clone() }
}

pub fn insert_font_handle(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut game_state: ResMut<NextState<GameState>>
) {
    commands.insert_resource(GlobalFont(assets.load("fonts/burbank.otf")));

    game_state.set(GameState::MainMenu);
}