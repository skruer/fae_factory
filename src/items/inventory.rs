use bevy::{prelude::*, utils::HashMap};

use crate::recipes::Recipe;

use super::ItemType;

#[derive(Component, Debug, Reflect)]
pub struct Inventory {
    pub items: HashMap<ItemType, u32>,
    pub slots: u16,
    pub input_filter: InventoryFilter,
    pub output_filter: InventoryFilter,
}

#[derive(Component, Debug, Reflect)]
pub enum InventoryFilter {
    All,
    Only(Vec<ItemType>),
    Except(Vec<ItemType>),
}

impl Inventory {
    pub fn new(slots: u16, items: Option<Vec<(ItemType, u32)>>) -> Self {
        let mut inventory = Inventory {
            items: HashMap::default(),
            slots,
            input_filter: InventoryFilter::All,
            output_filter: InventoryFilter::All,
        };
        match items {
            Some(items) => inventory.add_items(items),
            None => (),
        }
        inventory
    }

    pub fn filtered_for(mut self, recipe: &Recipe) -> Self {
        let inputs: Vec<ItemType> = recipe.input.iter().map(|(i, _)| *i).collect();
        self.input_filter = InventoryFilter::Only(inputs.clone());
        self.output_filter = InventoryFilter::Except(inputs);
        self
    }

    pub fn has_items(&self, items: Vec<(ItemType, u32)>) -> bool {
        items
            .iter()
            .all(|(item, amount)| self.has_item(item, *amount))
    }

    pub fn has_item(&self, item: &ItemType, amount: u32) -> bool {
        self.items.get(item).map_or(false, |a| *a >= amount)
    }

    pub fn add_items(&mut self, items: Vec<(ItemType, u32)>) {
        items.iter().for_each(|(item, amount)| {
            if let Some(current) = self.items.get_mut(item) {
                *current += amount;
            } else {
                self.items.insert(item.clone(), *amount);
            }
        });
        println!("{:?}", self.items);
    }

    pub fn remove_items(&mut self, items: Vec<(ItemType, u32)>) -> bool {
        if self.has_items(items.clone()) {
            items.iter().for_each(|(item, amount)| {
                if let Some(current) = self.items.get_mut(item) {
                    *current -= amount;
                    if *current == 0 {
                        self.items.remove(item);
                    }
                } else {
                    // Guaranteed to have item, so this should never happen.
                }
            });
            println!("{:?}", self.items);
            return true;
        }
        return false;
    }

    pub fn can_add_items(&self, items: &[ItemType]) -> bool {
        match &self.input_filter {
            InventoryFilter::All => true,
            InventoryFilter::Only(allowed) => items.iter().all(|item| allowed.contains(item)),
            InventoryFilter::Except(disallowed) => {
                items.iter().all(|item| !disallowed.contains(item))
            }
        }
    }

    pub fn pullable_items(&self, items: Vec<ItemType>) -> Vec<ItemType> {
        match &self.output_filter {
            InventoryFilter::All => items,
            InventoryFilter::Only(allowed) => items
                .into_iter()
                .filter(|item| allowed.contains(item))
                .collect(),
            InventoryFilter::Except(disallowed) => items
                .into_iter()
                .filter(|item| !disallowed.contains(item))
                .collect(),
        }
    }

    pub fn empty_into_other(&mut self, other: &mut Inventory) {
        let items_to_pull = self.pullable_items(self.items.iter().map(|(item, _)| *item).collect());
        other.add_items(
            self.items
                .iter()
                .filter(|(item, _)| items_to_pull.contains(item))
                .map(|(item, amount)| (*item, *amount))
                .collect(),
        );
        items_to_pull.into_iter().for_each(|item| {
            self.items.remove(&item);
        });
        println!("{:?}", self.items)
    }
}
