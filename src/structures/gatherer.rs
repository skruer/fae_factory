use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::Clickable,
    input::mouse,
    items::{inventory::Inventory, item_spawner::ItemSpawner, Item, ItemType},
    map::grid::GridPosition,
    recipes::{Recipe, RecipeType},
    structures::{Structure, StructureType, STRUCTURE_Z},
};

pub struct GathererStructurePlugin;

impl StructureType {
    pub fn get_gathering_spawner(&self) -> Option<ItemSpawner> {
        use ItemType::*;
        use StructureType::*;
        match self {
            WoodFairy => Some(ItemSpawner::new(vec![(Wood, 1)], 10.0)),
            StoneFairy => Some(ItemSpawner::new(vec![(Stone, 1)], 10.0)),
            CrystalFairy => Some(ItemSpawner::new(vec![(Crystal, 1)], 15.0)),
            _ => None,
        }
    }
}

#[derive(Bundle, Default)]
pub struct GathererBundle {
    pub spawner: ItemSpawner,
    pub inventory: Inventory,
}

impl Plugin for GathererStructurePlugin {
    fn build(&self, app: &mut App) {}
}

pub fn spawn_gatherer_structure(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    structure_type: StructureType,
    mouse_grid: &GridPosition,
) {
    use StructureType::{CrystalFairy, StoneFairy, WoodFairy};
    println!("Spawning gatherer at {:?}", mouse_grid);
    let texture = asset_server.load(structure_type.asset_file());
    let (name, display_text) = match structure_type {
        WoodFairy => ("Wood Fairy", "WOOD"),
        StoneFairy => ("Stone Fairy", "STONE"),
        CrystalFairy => ("Crystal Fairy", "CRYSTAL"),
        _ => return,
    };

    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: mouse_grid.sprite_translation_z(STRUCTURE_Z),
                    ..default()
                },
                ..default()
            },
            Name::new(name),
            Structure(structure_type),
            mouse_grid.clone(),
            structure_type.get_gathering_spawner().unwrap(),
            Inventory::new(2, None).filtered_only_remove().clone(),
            Clickable {},
        ))
        .with_children(|child_builder| {
            // Add the text section to the gatherer based on the type
            child_builder.spawn(Text2dBundle {
                text: Text::from_section(
                    display_text,
                    TextStyle {
                        font_size: 10.0,
                        color: Color::BLACK,
                        ..default()
                    },
                ),
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 1.0),
                    ..default()
                },
                text_anchor: Anchor::BottomCenter,
                ..default()
            });
        });
}
