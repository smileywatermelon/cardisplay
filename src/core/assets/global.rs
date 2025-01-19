use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GlobalFont(pub Handle<Font>);

impl GlobalFont {
    pub fn handle(&self) -> Handle<Font> {
        self.0.clone()
    }
}

pub fn insert_font_handle(
    mut commands: Commands,
    assets: Res<AssetServer>,
) {
    commands.insert_resource(GlobalFont(assets.load("fonts/burbank.otf")))
}