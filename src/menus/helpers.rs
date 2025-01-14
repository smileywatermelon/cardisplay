use bevy::prelude::*;
use crate::menus::mainmenu::MenuMarker;

pub(crate) fn button<S: ToString>(text: S) -> (Button, Text, Node, BackgroundColor){
    (
        Button,
        Text::new(text.to_string()),
        Node {
            display: Display::Flex,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            width: Val::Percent(20.0),
            height: Val::Percent(3.0),
            padding: UiRect::all(Val::Px(1.0)),
            border: UiRect::all(Val::Px(1.0)),
            ..default()
        },
        BackgroundColor(Color::srgb_u8(20, 22, 23)),
    )
}

#[derive(Event)]
pub struct DespawnMenu(pub bool);

pub(crate) fn despawn_menu(
    mut commands: Commands,
    menu: Query<Entity, With<MenuMarker>>,
    mut despawn: EventReader<DespawnMenu>
) {
    for ev in despawn.read() {
        if ev.0 {
            commands.entity(menu.single()).despawn_descendants();
            println!("Despawning menu");
        }
    }
}