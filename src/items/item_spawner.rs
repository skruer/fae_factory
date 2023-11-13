use std::{arch::x86_64::_mm256_add_epi16, thread::spawn};

use bevy::prelude::*;

use crate::structures::StructureType;

use super::{inventory::Inventory, ItemType};

pub struct ItemSpawnerPlugin;

impl Plugin for ItemSpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_item_into_inventory)
            .add_event::<ItemSpawnEvent>()
            .register_type::<ItemSpawner>();
    }
}

#[derive(Component, Debug, Reflect, Default)]
pub struct ItemSpawner {
    pub output: Vec<(ItemType, u32)>,
    pub interval_seconds: f32,
    pub timer: Timer,
}

impl TryFrom<StructureType> for ItemSpawner {
    type Error = ();
    fn try_from(value: StructureType) -> Result<Self, Self::Error> {
        use StructureType::{CrystalFairy, StoneFairy, WoodFairy};
        match value {
            WoodFairy => Ok(ItemSpawner::new(vec![(ItemType::Wood, 1)], 10.0)),
            StoneFairy => Ok(ItemSpawner::new(vec![(ItemType::Stone, 1)], 10.0)),
            CrystalFairy => Ok(ItemSpawner::new(vec![(ItemType::Crystal, 1)], 10.0)),
            _ => Err(()),
        }
    }
}

#[derive(Event, Debug, Reflect)]
pub struct ItemSpawnEvent {
    pub entity: Entity,
    pub items: Vec<(ItemType, u32)>,
}

#[derive(Component, Debug, Reflect)]
pub struct ItemSpawnSource(Inventory);

#[derive(Component, Debug, Reflect, Clone, Copy)]
pub struct ItemSpawnSpeed(pub f32);

impl ItemSpawner {
    pub fn new(output: Vec<(ItemType, u32)>, interval_seconds: f32) -> Self {
        ItemSpawner {
            output,
            interval_seconds,
            timer: Timer::from_seconds(interval_seconds, TimerMode::Repeating),
        }
    }
}

fn spawn_item_into_inventory(
    mut spawners: Query<(
        Entity,
        &mut ItemSpawner,
        Option<&ItemSpawnSpeed>,
        Option<&mut ItemSpawnSource>,
        &mut Inventory,
    )>,
    mut event: EventWriter<ItemSpawnEvent>,
    time: Res<Time>,
) {
    for (entity, mut spawner, speed, source, mut inventory) in spawners.iter_mut() {
        let spawn_speed = speed.map_or(1.0, |s| s.0);
        if spawner
            .timer
            .tick(time.delta().mul_f32(spawn_speed))
            .finished()
        {
            let items_to_add = match source {
                Some(mut source_inventory) => {
                    source_inventory.0.remove_if_possible(&spawner.output)
                }
                None => spawner.output.clone(),
            };
            inventory.add_items(&items_to_add);
            event.send(ItemSpawnEvent {
                entity,
                items: items_to_add,
            });
        };
    }
}
