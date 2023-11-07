use core::fmt;

use bevy::{input::mouse::MouseButtonInput, prelude::*, utils::HashMap, window::PrimaryWindow};

use crate::{
    input::MyWorldCoords,
    items::{Inventory, ItemId, ItemList},
    player::Player,
};

pub struct StructurePlugin;

impl Plugin for StructurePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (spawn_structure, select_structure))
            .register_type::<Structure>()
            .register_type::<StructureId>()
            .register_type::<StructureList>();
    }
}

#[derive(Component, Reflect)]
struct Structure {
    structure_id: StructureId,
}

#[derive(Component, Reflect)]
pub struct SelectedStructure(pub Option<StructureId>);

#[derive(PartialEq, Eq, Hash, Clone, Debug, Reflect)]
pub struct StructureId(String);

impl StructureId {
    fn new(id: StructureList) -> Self {
        StructureId(id.to_string())
    }
}

#[derive(Component, Debug, Reflect)]
enum StructureList {
    Assembler,
    Conveyor,
    Miner,
    Splitter,
    Storage,
}

impl fmt::Display for StructureList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use StructureList::*;
        match self {
            Assembler => write!(f, "assembler"),
            Conveyor => write!(f, "conveyor"),
            Miner => write!(f, "miner"),
            Splitter => write!(f, "splitter"),
            Storage => write!(f, "storage"),
        }
    }
}

fn spawn_structure(
    mut commands: Commands,
    mouse: Res<Input<MouseButton>>,
    mouse_position: Res<MyWorldCoords>,
    mut query: Query<(&mut Inventory, &Transform), With<Player>>,
    selected_structure: Query<&SelectedStructure>,
    asset_server: Res<AssetServer>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        println!("Left mouse button pressed");
        let (mut inventory, transform) = query.single_mut();
        let structure_id = selected_structure.single().0.clone();

        if structure_id.is_none() {
            return;
        }
        let structure_id = structure_id.unwrap();

        if inventory.remove_item(&ItemId::new(ItemList::Crystal), 1) {
            let mut position = mouse_position.0.round();
            position =
            let texture = asset_server.load("Building.png");
            println!(
                "Spawning structure at {:?}; player {:?}",
                position, transform,
            );
            commands.spawn((
                SpriteBundle {
                    texture,
                    transform: Transform {
                        translation: Vec3::new(position.x, position.y, 0.0),
                        ..default()
                    },
                    ..default()
                },
                Name::new(structure_id.0.clone()),
                Structure { structure_id },
            ));
        }
    }
}

fn select_structure(keys: Res<Input<KeyCode>>, mut query: Query<&mut SelectedStructure>) {
    // This should be defined like this for future use with player configuration
    let select_keys = vec![
        (KeyCode::Key1, StructureId::new(StructureList::Assembler)),
        (KeyCode::Key2, StructureId::new(StructureList::Conveyor)),
        (KeyCode::Key3, StructureId::new(StructureList::Miner)),
        (KeyCode::Key4, StructureId::new(StructureList::Splitter)),
        (KeyCode::Key5, StructureId::new(StructureList::Storage)),
    ];

    let mut selected_structure = query.single_mut();

    select_keys.iter().for_each(|(key, structure)| {
        if keys.just_pressed(*key) {
            selected_structure.0 = Some(structure.clone());
            println!("Selected structure: {}", structure.0);
        }
    });
}
