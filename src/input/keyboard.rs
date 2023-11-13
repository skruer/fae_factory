use bevy::{prelude::*, utils::HashMap};

use crate::{
    common::{Held, Holdable},
    items::ItemType,
    player::events::PlayerMoveEvent,
    structures::StructureType,
};

pub(super) struct FaeKeyboardPlugin;

impl Plugin for FaeKeyboardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_movement_input, select_held))
            .insert_resource(HeldState::default());
    }
}

#[derive(Resource, Reflect, Debug, Default)]
pub struct HeldState {
    pub key: Option<KeyCode>,
    pub index: usize,
}

pub(super) fn handle_movement_input(
    key_input: Res<Input<KeyCode>>,
    mut event: EventWriter<PlayerMoveEvent>,
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
    direction.map(|direction| event.send(PlayerMoveEvent(direction)));
}

pub(super) fn select_held(
    keys: Res<Input<KeyCode>>,
    mut query: Query<&mut Held>,
    mut held_state: ResMut<HeldState>,
) {
    // This should be defined like this for future use with player configuration
    use Holdable::*;
    use ItemType::*;
    use StructureType::*;
    let select_keys: HashMap<KeyCode, Vec<Holdable>> = [
        (KeyCode::Key1, vec![Structure(Assembler)]),
        (
            KeyCode::Key2,
            vec![Structure(Conveyor), Structure(Grabber), Structure(Chest)],
        ),
        (
            KeyCode::Key3,
            vec![
                Structure(WoodFairy),
                Structure(StoneFairy),
                Structure(CrystalFairy),
            ],
        ),
        (KeyCode::Key4, vec![Item(Wood), Item(Stone), Item(Crystal)]),
        (KeyCode::Key0, vec![]),
    ]
    .into_iter()
    .collect();

    let selected_key = select_keys
        .iter()
        .filter_map(|(key, structures)| match keys.just_pressed(*key) {
            true => Some((key, structures)),
            _ => None,
        })
        .last();

    *held_state = match selected_key {
        Some(key_info) => HeldState {
            key: Some(*key_info.0),
            index: match held_state.key {
                Some(key) if key == *key_info.0 => (held_state.index + 1) % key_info.1.len(),
                _ => 0,
            },
        },
        _ => return,
    };

    let mut held = query.single_mut();
    let holdable = select_keys
        .get(&held_state.key.map_or(KeyCode::Escape, |key| key))
        .map(|structures| structures.get(held_state.index))
        .flatten();
    *held = Held(holdable.cloned());
    println!("Held: {:?}", held);
}
