use core::fmt;

use bevy::{prelude::*, sprite::Anchor};

use crate::{
    common::{round_to_grid, Clickable, Held, Holdable, Hoverable},
    input::{
        mouse::{FaeEntityClickEvent, FaeEntityContextClickEvent},
        MyWorldCoords,
    },
    items::{inventory::Inventory, ItemType},
    map::grid::{GridPosition, HoveredGrid},
    player::Player,
    structures::{
        assembler::{spawn_assembler, AssemblerBundle},
        chest::ChestBundle,
        gatherer::{spawn_gatherer_structure, GathererBundle},
    },
};

use self::assembler::AssemblerPlugin;

pub mod assembler;
pub mod chest;
pub mod gatherer;

const STRUCTURE_Z: f32 = 1.0;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssemblerPlugin)
            .add_systems(Update, (handle_spawn_structure, handle_remove_structure))
            .register_type::<Structure>()
            .register_type::<StructureType>();
    }
}

#[derive(Component, Reflect, Default)]
pub struct Structure(pub StructureType);

#[derive(Bundle, Default)]
pub struct StructureBundle {
    pub structure: Structure,
    pub grid_position: GridPosition,
    pub clickable: Clickable,
    pub hoverable: Hoverable,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect, Default)]
pub enum StructureType {
    #[default]
    Assembler,
    Conveyor,
    Chest,
    Grabber,
    WoodFairy,
    StoneFairy,
    CrystalFairy,
}

impl StructureType {
    fn get_cost(&self) -> Vec<(ItemType, u32)> {
        use ItemType::*;
        use StructureType::*;
        match self {
            Assembler => vec![(Crystal, 3), (Wood, 3)],
            Conveyor => vec![(Crystal, 1), (Wood, 1), (Stone, 1)],
            Chest => vec![(Stone, 5)],
            Grabber => vec![(Crystal, 2)],
            WoodFairy => vec![(Toy, 2)],
            StoneFairy => vec![(Toy, 3)],
            CrystalFairy => vec![(Toy, 5)],
        }
    }

    fn name(&self) -> String {
        match self {
            _ => format!("{:?}", self),
        }
    }

    fn debug_marker(&self) -> &'static str {
        use StructureType::*;
        match self {
            Assembler => "CRFT",
            Conveyor => ">>>",
            Chest => "CHST",
            Grabber => "ARM",
            WoodFairy => "WOOD",
            StoneFairy => "STNE",
            CrystalFairy => "CSTL",
        }
    }

    fn asset_file(&self) -> String {
        match self {
            _ => format!("building_{}.png", GridPosition::PIXELS_PER_TILE),
        }
    }
}

impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StructureType::*;
        match self {
            Assembler => write!(f, "assembler"),
            Conveyor => write!(f, "conveyor"),
            Chest => write!(f, "storage"),
            Grabber => write!(f, "grabber"),
            WoodFairy => write!(f, "wood-fairy"),
            StoneFairy => write!(f, "stone-fairy"),
            CrystalFairy => write!(f, "crystal-fairy"),
        }
    }
}

fn handle_spawn_structure(
    mut commands: Commands,
    mut event: EventReader<FaeEntityClickEvent>,
    mouse_grid: Res<HoveredGrid>,
    mut query: Query<(&mut Inventory, &GridPosition), With<Player>>,
    mut selected_structure: Query<&mut Held>,
    asset_server: Res<AssetServer>,
) {
    let mut selected_structure = selected_structure.single_mut();
    if let Some(click_event) = event.iter().last() {
        if click_event.entities.len() > 0 {
            // If this tile is occupied, don't spawn a structure.
            // TODO: Query the tile to see if it can support a structure?
            return;
        }

        let (mut inventory, _player_grid) = query.single_mut();

        let structure_type = match selected_structure.0 {
            Some(Holdable::Structure(structure_type)) => structure_type,
            _ => return,
        };
        println!(
            "Spawning structure {:?} for {:?}",
            structure_type,
            structure_type.get_cost()
        );
        if inventory.remove_items(&structure_type.get_cost()) {
            use StructureType::*;
            let mut structure_commands = commands.spawn((
                StructureBundle {
                    structure: Structure(structure_type),
                    grid_position: mouse_grid.0.clone(),
                    ..default()
                },
                SpriteBundle {
                    transform: Transform {
                        translation: mouse_grid.0.sprite_translation_z(STRUCTURE_Z),
                        ..default()
                    },
                    texture: asset_server.load(structure_type.asset_file()),
                    ..default()
                },
                Name::from(structure_type.name()),
            ));
            // Add the debug marker text to identify the structure.
            structure_commands.with_children(|child_builder| {
                child_builder.spawn(Text2dBundle {
                    text: Text::from_section(
                        structure_type.debug_marker(),
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
            match structure_type {
                Assembler => {
                    structure_commands.insert(AssemblerBundle::default());
                }
                Chest => {
                    structure_commands.insert(ChestBundle::default());
                }
                WoodFairy | StoneFairy | CrystalFairy => {
                    structure_commands.insert(GathererBundle {
                        spawner: structure_type.get_gathering_spawner().unwrap(),
                        ..default()
                    });
                }
                _ => (),
            }
            *selected_structure = Held(None);
        }
    }
}

fn handle_remove_structure(
    mut commands: Commands,
    mut event: EventReader<FaeEntityContextClickEvent>,
    mut query: Query<(&mut Inventory, &GridPosition), With<Player>>,
    mut selected_structure: Query<&mut Held>,
    mut structure: Query<(&Structure, Option<&mut Inventory>), (With<Clickable>, Without<Player>)>,
) {
    if let Some(Holdable::Item(_)) = selected_structure.single_mut().0 {
        // If we're holding an item, we don't want to remove a structure.
        return;
    }

    if let Some(event) = event.iter().last() {
        if !event.modifiers.check_only_pressed(&vec![]) || event.entities.is_empty() {
            return;
        }

        let entity = event.entities.first().unwrap();
        let (mut player_inventory, _player_grid) = query.single_mut();
        if let Ok((structure, mut structure_inventory)) = structure.get_mut(*entity) {
            player_inventory.add_items(&structure.0.get_cost());
            if let Some(structure_inventory) = structure_inventory.as_mut() {
                structure_inventory.force_empty_into_other(player_inventory.as_mut());
            }
            // Remove the structure that was clicked and its descendent entities to clear text
            commands.entity(*entity).despawn_recursive();
        }
    }
}
