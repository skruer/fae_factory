// TODO: Determine how to handle the organization of this bit.

use bevy::{ecs::reflect, prelude::*, utils::HashMap};
use core::fmt;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ItemId {
    id: String,
}

impl ItemId {
    pub fn new(id: ItemList) -> Self {
        ItemId { id: id.to_string() }
    }
}

// TODO: Think on this some more
pub enum ItemList {
    Wood,
    Crystal,
    Stone,
    Toy,
}

impl fmt::Display for ItemList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ItemList::Wood => write!(f, "core::wood"),
            ItemList::Crystal => write!(f, "core::crystal"),
            ItemList::Stone => write!(f, "core::stone"),
            ItemList::Toy => write!(f, "core::toy"),
        }
    }
}

#[derive(Component, Debug)]
pub struct Inventory {
    pub items: HashMap<ItemId, u32>,
    pub slots: u16,
}

impl Inventory {
    pub fn new(slots: u16, collection: impl IntoIterator<Item = (ItemId, u32)>) -> Self {
        let mut inventory = Inventory {
            items: HashMap::default(),
            slots,
        };
        for (item, amount) in collection {
            inventory.add_item(&item, amount);
        }
        inventory
    }

    pub fn has_item(&self, item: &ItemId, amount: u32) -> bool {
        self.items.get(item).map_or(false, |a| *a >= amount)
    }

    pub fn add_item(&mut self, item: &ItemId, amount: u32) {
        if self.has_item(item, 1) {
            for (i, a) in &mut self.items {
                if i.id == item.id {
                    *a += amount;
                    break;
                }
            }
        } else {
            self.items.insert(item.clone(), amount);
        }
        println!("{:?}", self.items);
    }

    pub fn remove_item(&mut self, item: &ItemId, amount: u32) {
        if self.items.get(item).map_or(false, |a| *a >= amount) {
            for (i, a) in &mut self.items {
                if i.id == item.id {
                    *a -= amount;
                    break;
                }
            }
        }
        println!("{:?}", self.items);
    }
}
