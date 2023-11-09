use bevy::prelude::*;

use crate::{
    common::{Held, Holdable},
    items::ItemType,
    player::{Player, PlayerMove},
    structures::StructureType,
};

pub(super) struct FaeKeyboardPlugin;

impl Plugin for FaeKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_movement_input, select_held));
    }
}

pub(super) fn handle_movement_input(
    key_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut PlayerMove,), With<Player>>,
) {
    let keys = vec![
        (KeyCode::W, Vec2::new(0.0, 1.0)),
        (KeyCode::S, Vec2::new(0.0, -1.0)),
        (KeyCode::D, Vec2::new(1.0, 0.0)),
        (KeyCode::A, Vec2::new(-1.0, 0.0)),
    ];

    let direction = keys
        .iter()
        .map(|(key, direction)| match key_input.pressed(*key) {
            true => *direction,
            _ => Vec2::new(0.0, 0.0),
        })
        .fold(Vec2::new(0.0, 0.0), |acc, direction| acc + direction)
        .try_normalize();
    let mut player = player.single_mut();
    *player.0 = PlayerMove(direction);
}

pub(super) fn select_held(keys: Res<Input<KeyCode>>, mut query: Query<&mut Held>) {
    // This should be defined like this for future use with player configuration
    use Holdable::*;
    let select_keys = vec![
        (KeyCode::Key1, Structure(StructureType::Assembler)),
        (KeyCode::Key2, Structure(StructureType::Conveyor)),
        (KeyCode::Key3, Structure(StructureType::Storage)),
        (KeyCode::Key4, Structure(StructureType::Grabber)),
        (KeyCode::Key5, Item(ItemType::Wood)),
        (KeyCode::Key6, Item(ItemType::Stone)),
        (KeyCode::Key7, Item(ItemType::Crystal)),
        (KeyCode::Key8, Item(ItemType::Toy)),
    ];

    let mut selected_structure = query.single_mut();

    select_keys.iter().for_each(|(key, structure)| {
        if keys.just_pressed(*key) {
            *selected_structure = Held(Some(*structure));
            println!("Selected structure: {:?}", structure);
        }
    });
}
