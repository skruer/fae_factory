use bevy::prelude::*;

use crate::common::Held;
use crate::input::mouse::FaeEntityClickEvent;
use crate::input::FaeEntityInputModifier;
use crate::items::inventory::Inventory;
use crate::research::AvailableRecipes;
use crate::{
    common::Clickable,
    crafting::{Crafter, CrafterState},
    recipes::{Recipe, RecipeType},
};

use super::{Structure, StructureType, STRUCTURE_Z};

pub(super) struct AssemblerPlugin;

impl Plugin for AssemblerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, select_assembler_recipe);
    }
}

pub(super) fn spawn_assembler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: &Vec2,
) {
    println!("Spawning assembler at {:?}", position);
    let texture = asset_server.load("building.png");
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform {
                translation: Vec3::new(position.x, position.y, STRUCTURE_Z),
                ..default()
            },
            ..default()
        },
        Name::new("Assembler"),
        Structure {
            structure_type: StructureType::Assembler,
        },
        Crafter {
            recipe: None,
            state: CrafterState::Pending(true),
            ..default()
        },
        Inventory::new(2, None).filtered_for(&Recipe::from(RecipeType::WoodToToy)),
        Clickable {},
    ));
}

fn select_assembler_recipe(
    mut event: EventReader<FaeEntityClickEvent>,
    mut query: Query<&mut Crafter>,
    available_recipes: Res<AvailableRecipes>,
    selected_structure: Query<&Held>,
) {
    if let Some(_) = selected_structure.single().0 {
        return;
    }

    if let Some(event) = event.into_iter().last() {
        let entity = match event.entity {
            Some(entity) => entity,
            None => return,
        };

        if event
            .modifiers
            .check_only_pressed(&vec![FaeEntityInputModifier::Shift])
        {
            println!("Selecting assembler recipe");
            if let Ok(mut crafter) = query.get_mut(entity) {
                crafter.recipe = match crafter.recipe.as_ref() {
                    Some(recipe) => Some(Recipe::from(
                        recipe.recipe_type.next_available_recipe(&available_recipes),
                    )),
                    None => Some(Recipe::from(RecipeType::WoodToToy)),
                };
            }
        }
    }
}
