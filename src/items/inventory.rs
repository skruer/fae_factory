use bevy::{prelude::*, utils::HashMap};

use crate::recipes::Recipe;

use super::ItemType;

#[derive(Component, Debug, Reflect, Clone)]
pub struct Inventory {
    pub items: HashMap<ItemType, u32>,
    pub slots: u16,
    pub input_filter: InventoryFilter,
    pub output_filter: InventoryFilter,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            items: HashMap::default(),
            slots: 10,
            input_filter: InventoryFilter::None,
            output_filter: InventoryFilter::All,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Reflect, Component)]
pub struct ItemAmount {
    pub item: ItemType,
    pub amount: Option<u32>,
}

impl ItemAmount {
    fn to_tuple(&self) -> (ItemType, u32) {
        (self.item, self.amount.unwrap_or(0))
    }
}

impl From<(ItemType, u32)> for ItemAmount {
    fn from((item, amount): (ItemType, u32)) -> Self {
        ItemAmount {
            item,
            amount: Some(amount),
        }
    }
}

impl Into<(ItemType, u32)> for ItemAmount {
    fn into(self) -> (ItemType, u32) {
        (self.item, self.amount.unwrap_or(0))
    }
}

#[derive(Component, Debug, Reflect, Clone)]
pub enum InventoryFilter {
    All,
    Only(Vec<ItemType>),
    Except(Vec<ItemType>),
    None,
}

impl Inventory {
    pub fn new(slots: u16, items: Vec<ItemAmount>) -> Self {
        let mut inventory = Inventory {
            items: HashMap::default(),
            slots,
            input_filter: InventoryFilter::All,
            output_filter: InventoryFilter::All,
        };
        items.into_iter().for_each(|item_amount| {
            let (item, amount) = item_amount.into();
            inventory.items.insert(item, amount);
        });
        inventory
    }

    pub fn filtered_for(&mut self, recipe: Option<&Recipe>) -> &Self {
        let inputs: Option<Vec<ItemType>> = recipe.map(|recipe| {
            recipe
                .input
                .iter()
                .map(|item_amount| item_amount.item)
                .collect()
        });
        if let Some(inputs) = inputs {
            self.input_filter = InventoryFilter::Only(inputs.clone());
            self.output_filter = InventoryFilter::Except(inputs);
        } else {
            // Nothing can go in without a recipe
            self.input_filter = InventoryFilter::None;
            self.output_filter = InventoryFilter::All;
        }
        self
    }

    pub fn clear_filters(mut self) -> Self {
        self.input_filter = InventoryFilter::All;
        self.output_filter = InventoryFilter::All;
        self
    }

    pub fn filtered_only_remove(&mut self) -> &Self {
        self.input_filter = InventoryFilter::None;
        self.output_filter = InventoryFilter::All;
        self
    }

    pub fn has_items(&self, items: &Vec<ItemAmount>) -> bool {
        items.iter().all(|item_amount| self.has_item(item_amount))
    }

    pub fn has_item(&self, item_amount: &ItemAmount) -> bool {
        self.items
            .get(&item_amount.item)
            .map_or(false, |a| *a >= item_amount.amount.map_or(0, |a| a))
    }

    pub fn add_items(&mut self, items: &Vec<ItemAmount>) {
        items.iter().for_each(|item_amount| {
            let (item, amount) = item_amount.to_tuple();
            if let Some(current) = self.items.get_mut(&item) {
                *current += amount;
            } else {
                self.items.insert(item.clone(), amount);
            }
        });
        println!("{:?}", self.items);
    }

    pub fn remove_items(&mut self, items: &Vec<ItemAmount>) -> bool {
        if self.has_items(items) {
            items.iter().for_each(|item_amount| {
                let (item, amount) = item_amount.to_tuple();
                if let Some(current) = self.items.get_mut(&item) {
                    *current -= amount;
                    if *current == 0 {
                        self.items.remove(&item);
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

    pub fn remove_if_possible(&mut self, items: &Vec<ItemAmount>) -> Vec<ItemAmount> {
        items
            .iter()
            .filter_map(|item_amount| {
                let (item, amount) = item_amount.to_tuple();
                match self.items.get_mut(&item) {
                    Some(current) if *current > 0 => {
                        let removed = if *current >= amount {
                            *current -= amount;
                            amount
                        } else {
                            self.items.remove(&item).unwrap()
                        };
                        Some(ItemAmount {
                            item,
                            amount: Some(removed),
                        })
                    }
                    _ => None,
                }
            })
            .collect()
    }

    pub fn can_add_items(&self, items: &[ItemType]) -> bool {
        use InventoryFilter::*;
        match &self.input_filter {
            All => true,
            Only(allowed) => items.iter().all(|item| allowed.contains(item)),
            Except(disallowed) => items.iter().all(|item| !disallowed.contains(item)),
            None => false,
        }
    }

    pub fn pullable_items(&self, items: Vec<ItemAmount>) -> Vec<ItemAmount> {
        match &self.output_filter {
            InventoryFilter::All => items,
            InventoryFilter::Only(allowed) => items
                .into_iter()
                .filter(|item| allowed.contains(&item.item))
                .collect(),
            InventoryFilter::Except(disallowed) => items
                .into_iter()
                .filter(|item| !disallowed.contains(&item.item))
                .collect(),
            InventoryFilter::None => vec![], // This should probably not be possible, encode it in the type lol
        }
    }

    pub fn try_empty_into_other(&mut self, other: &mut Inventory) {
        // Empty, respecting filters
        let items_to_pull = self.pullable_items(
            self.items
                .iter()
                .map(|(item, amount)| (*item, *amount).into())
                .collect(),
        );
        self.move_items_into_other(other, items_to_pull);
    }

    pub fn force_empty_into_other(&mut self, other: &mut Inventory) -> &mut Self {
        // Skip filtering
        let items_to_pull = self
            .items
            .iter()
            .map(|(item, amount)| (*item, *amount).into())
            .collect();
        self.move_items_into_other(other, items_to_pull);
        self
    }

    pub fn move_items_into_other(&mut self, other: &mut Inventory, items_to_pull: Vec<ItemAmount>) {
        // Generic function to move items between inventories by type
        let pulled_types: Vec<ItemType> = items_to_pull
            .iter()
            .map(|item_amount| item_amount.item)
            .collect();

        other.add_items(
            &self
                .items
                .iter()
                .filter(|(item, _)| pulled_types.contains(item))
                .map(|(item, amount)| (*item, *amount).into())
                .collect(),
        );
        items_to_pull.into_iter().for_each(|item| {
            self.items.remove(&item.item);
        });
        println!("{:?}", self.items)
    }
}
