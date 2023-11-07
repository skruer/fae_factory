// TODO: Determine how to handle the organization of this bit.

use bevy::{prelude::*, utils::HashMap};
use core::fmt;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Item>()
            .register_type::<Inventory>()
            .register_type::<ItemId>();
    }
}

// This is going to be for items that are in the world.
#[derive(Component, Reflect)]
pub struct Item {
    pub id: ItemId,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug, Reflect)]
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
        use ItemList::*;
        match self {
            Wood => write!(f, "wood"),
            Crystal => write!(f, "crystal"),
            Stone => write!(f, "stone"),
            Toy => write!(f, "toy"),
        }
    }
}

#[derive(Component, Debug, Reflect)]
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

    pub fn remove_item(&mut self, item: &ItemId, amount: u32) -> bool {
        if let Some(current) = self.items.get_mut(item) {
            if *current >= amount {
                *current -= amount;
                println!("{:?}", self.items);
                return true;
            }
        }
        println!("{:?}", self.items);
        return false;
    }
}
