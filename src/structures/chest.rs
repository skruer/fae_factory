use bevy::prelude::*;

use crate::items::inventory::Inventory;

pub struct ChestPlugin;

impl Plugin for ChestPlugin {
    fn build(&self, app: &mut App) {}
}

#[derive(Component, Reflect, Default)]
pub struct Chest;

#[derive(Bundle)]
pub struct ChestBundle {
    pub chest: Chest,
    pub inventory: Inventory,
}

impl Default for ChestBundle {
    fn default() -> Self {
        ChestBundle {
            chest: Chest,
            inventory: Inventory::new(10, vec![]).clear_filters(),
        }
    }
}
