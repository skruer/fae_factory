// TODO: Determine how to handle the organization of this bit.

use bevy::prelude::*;
use core::fmt;

use crate::{
    common::{Clickable, Held, Holdable},
    input::{
        mouse::{FaeEntityClickEvent, FaeEntityContextClickEvent},
        FaeEntityInputModifier, FaeInputModifier,
    },
    player::Player,
};

use self::{inventory::Inventory, item_spawner::ItemSpawnerPlugin};

pub(crate) mod inventory;
pub(crate) mod item_spawner;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ItemSpawnerPlugin)
            .add_systems(Update, (handle_click_insert_item, handle_click_empty))
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
    let (mut clicked_inventory, _clicked_transform) = match event.iter().last() {
        Some(click_event) => {
            if !click_event.modifiers.check_only_pressed(&vec![]) || click_event.entities.is_empty()
            {
                return;
            }

            let entity = click_event.entities.first().unwrap();
            if let Ok((inventory, transform)) = query.get_mut(*entity) {
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

    let (mut player_inventory, _player_transform) = player.single_mut();
    if player_inventory.remove_items(&vec![(item, 1)]) {
        clicked_inventory.add_items(&vec![(item, 1)]);
    }

    if !player_inventory.has_item(&item, 1) {
        *held = Held(None);
    }
}

fn handle_click_empty(
    mut event: EventReader<FaeEntityClickEvent>,
    mut player: Query<(&mut Inventory, &Transform), With<Player>>,
    mut query: Query<(&mut Inventory, &Transform), (With<Clickable>, Without<Player>)>,
) {
    let (mut clicked_inventory, _clicked_transform) = match event.iter().last() {
        Some(click_event) => {
            if !click_event
                .modifiers
                .check_only_pressed(&vec![FaeEntityInputModifier::Ctrl])
                || click_event.entities.is_empty()
            {
                return;
            }

            let entity = click_event.entities.first().unwrap();
            if let Ok((inventory, transform)) = query.get_mut(*entity) {
                (inventory, transform)
            } else {
                return;
            }
        }
        _ => return,
    };
    println!("Emptying inventory to player");

    let (mut player_inventory, _player_transform) = player.single_mut();
    clicked_inventory.try_empty_into_other(&mut player_inventory);
}
