use core::fmt;

use bevy::prelude::*;

use crate::{
    common::{round_to_grid, BoundingBox, Clickable, Held, Holdable},
    input::{mouse::FaeEntityClickEvent, MyWorldCoords},
    items::{inventory::Inventory, ItemType},
    player::Player,
    structures::assembler::spawn_assembler,
};

use self::assembler::AssemblerPlugin;

mod assembler;

const STRUCTURE_Z: f32 = 1.0;

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AssemblerPlugin)
            .add_systems(Update, (spawn_structure, bound_structure))
            .register_type::<Structure>()
            .register_type::<StructureType>();
    }
}

#[derive(Component, Reflect)]
struct Structure {
    structure_type: StructureType,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum StructureType {
    Assembler,
    Conveyor,
    Storage,
    Grabber,
}

impl StructureType {
    fn get_cost(&self) -> Vec<(ItemType, u32)> {
        use ItemType::*;
        use StructureType::*;
        match self {
            Assembler => [(Crystal, 3), (Wood, 3)].to_vec(),
            Conveyor => [(Crystal, 1), (Wood, 1), (Stone, 1)].to_vec(),
            Storage => [(Stone, 5)].to_vec(),
            Grabber => [(Crystal, 2)].to_vec(),
        }
    }
}

impl fmt::Display for StructureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StructureType::*;
        match self {
            Assembler => write!(f, "assembler"),
            Conveyor => write!(f, "conveyor"),
            Storage => write!(f, "storage"),
            Grabber => write!(f, "grabber"),
        }
    }
}

fn spawn_structure(
    commands: Commands,
    mut event: EventReader<FaeEntityClickEvent>,
    mouse_position: Res<MyWorldCoords>,
    mut query: Query<(&mut Inventory, &Transform), With<Player>>,
    mut selected_structure: Query<&mut Held>,
    asset_server: Res<AssetServer>,
) {
    let mut selected_structure = selected_structure.single_mut();
    if let Some(click_event) = event.iter().last() {
        if let Some(_entity) = click_event.entity {
            // If we clicked on a structure, don't spawn a new one
            return;
        }

        let (mut inventory, _transform) = query.single_mut();

        let structure_type = match selected_structure.0 {
            Some(Holdable::Structure(structure_type)) => structure_type,
            _ => return,
        };
        if inventory.remove_items(structure_type.get_cost()) {
            let position = round_to_grid(mouse_position.0);
            match structure_type {
                StructureType::Assembler => spawn_assembler(commands, asset_server, &position),
                StructureType::Conveyor => {}
                StructureType::Storage => {}
                StructureType::Grabber => {}
            }
            *selected_structure = Held(None);
        }
    }
}

fn bound_structure(
    mut commands: Commands,
    structure: Query<(Entity, &Transform, &Handle<Image>), (With<Clickable>, Without<BoundingBox>)>,
    assets: Res<Assets<Image>>,
) {
    for (entity, transform, texture_handle) in &mut structure.iter() {
        if let Some(image) = assets.get(texture_handle) {
            let image_dimensions = image.size();
            let scaled_image_dimension = image_dimensions * transform.scale.truncate();
            let bounding_box = BoundingBox(Rect::from_center_size(
                transform.translation.truncate(),
                scaled_image_dimension,
            ));
            println!("Adding bounding box to structure at {:?}", bounding_box);
            commands.entity(entity).insert(bounding_box);
        }
    }
}
