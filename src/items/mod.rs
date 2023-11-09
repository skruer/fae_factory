// TODO: Determine how to handle the organization of this bit.

use bevy::{ecs::event, prelude::*, utils::HashMap};
use core::fmt;

use crate::{
    common::{Clickable, Held, Holdable},
    input::mouse::{FaeEntityClickEvent, FaeEntityContextClickEvent},
    player::Player,
    recipes::Recipe,
};

use self::inventory::Inventory;

pub(crate) mod inventory;
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_click_insert_item, handle_click_empty))
            .register_type::<Item>()
            .register_type::<Inventory>()
            .register_type::<ItemType>();
    }
}

// This is going to be for items that are in the world.
#[derive(Component, Reflect)]
pub struct Item(ItemType);

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ItemType {
    Wood,
    Crystal,
    Stone,
    Toy,
}

#[derive(Component, Debug, Reflect)]
pub struct ItemSpawner {
    pub item: ItemType,
    pub amount: u32,
    pub interval: f32,
    pub timer: Timer,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ItemType::*;
        match self {
            Wood => write!(f, "wood"),
            Crystal => write!(f, "crystal"),
            Stone => write!(f, "stone"),
            Toy => write!(f, "toy"),
        }
    }
}

fn handle_click_insert_item(
    mut event: EventReader<FaeEntityClickEvent>,
    mut player: Query<(&mut Inventory, &Transform), With<Player>>,
    mut query: Query<(&mut Inventory, &Transform), (With<Clickable>, Without<Player>)>,
    mut held_item: Query<&mut Held>,
) {
    // Retrieve the newest click event, if it exists, and extract the clicked inventory and transform.
    let (mut clicked_inventory, clicked_transform) = match event.iter().last() {
        Some(click_event) => {
            if !click_event.modifiers.check_only_pressed(&vec![]) || click_event.entity.is_none() {
                return;
            }

            let entity = click_event.entity.unwrap();
            if let Ok((inventory, transform)) = query.get_mut(entity) {
                (inventory, transform)
            } else {
                return;
            }
        }
        _ => return,
    };

    let mut held = held_item.single_mut();
    let item = match held.0 {
        Some(Holdable::Item(item)) => item,
        _ => return,
    };

    if !clicked_inventory.can_add_items(&[item]) {
        return;
    }

    let (mut player_inventory, player_transform) = player.single_mut();
    if player_inventory.remove_items([(item, 1)].to_vec()) {
        clicked_inventory.add_items([(item, 1)].to_vec());
    }

    if !player_inventory.has_items([(item, 1)].to_vec()) {
        *held = Held(None);
    }
}

fn handle_click_empty(
    mut event: EventReader<FaeEntityContextClickEvent>,
    mut player: Query<(&mut Inventory, &Transform), With<Player>>,
    mut query: Query<(&mut Inventory, &Transform), (With<Clickable>, Without<Player>)>,
) {
    let (mut clicked_inventory, clicked_transform) = match event.iter().last() {
        Some(click_event) => {
            if !click_event.modifiers.check_only_pressed(&vec![]) || click_event.entity.is_none() {
                return;
            }

            let entity = click_event.entity.unwrap();
            if let Ok((inventory, transform)) = query.get_mut(entity) {
                (inventory, transform)
            } else {
                return;
            }
        }
        _ => return,
    };

    let (mut player_inventory, player_transform) = player.single_mut();
    clicked_inventory.empty_into_other(&mut player_inventory);
}
