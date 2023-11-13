use bevy::prelude::*;
use bevy::render::render_phase::PhaseItem;
use bevy::sprite::Anchor;

use crate::common::Held;
use crate::input::mouse::FaeEntityClickEvent;
use crate::input::FaeEntityInputModifier;
use crate::items::inventory::Inventory;
use crate::items::ItemType;
use crate::map::grid::GridPosition;
use crate::player::Player;
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
        app.add_systems(
            Update,
            (
                select_assembler_recipe,
                handle_recipe_change.after(select_assembler_recipe),
            ),
        )
        .add_event::<AssemblerRecipeChangedEvent>();
    }
}

#[derive(Bundle, Default)]
pub struct AssemblerBundle {
    pub crafter: Crafter,
    pub inventory: Inventory,
}

#[derive(Event, Debug, Reflect)]
pub struct AssemblerRecipeChangedEvent(Entity);

pub(super) fn spawn_assembler(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    position: &GridPosition,
) {
    println!("Spawning assembler at {:?}", position);
    let texture = asset_server.load(StructureType::Assembler.asset_file());
    commands
        .spawn((
            SpriteBundle {
                texture,
                transform: Transform {
                    translation: position.sprite_translation_z(STRUCTURE_Z),
                    ..default()
                },
                ..default()
            },
            Name::new("Assembler"),
            Structure(StructureType::Assembler),
            Crafter {
                recipe: None,
                state: CrafterState::Pending(true),
                ..default()
            },
            Inventory::new(2, None).filtered_for(None).clone(),
            Clickable {},
        ))
        .with_children(|child_builder| {
            child_builder.spawn(Text2dBundle {
                text: Text::from_section(
                    "CRFT",
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

fn select_assembler_recipe(
    mut click_event: EventReader<FaeEntityClickEvent>,
    mut query: Query<&mut Crafter, Without<Player>>,
    mut player: Query<&mut Inventory, With<Player>>,
    available_recipes: Res<AvailableRecipes>,
    mut recipe_change_event: EventWriter<AssemblerRecipeChangedEvent>,
) {
    if let Some(event) = click_event.into_iter().last() {
        if event
            .modifiers
            .check_only_pressed(&vec![FaeEntityInputModifier::Shift])
        {
            println!("Selecting assembler recipe");
            // TODO: Rewrite this to be fucking sane
            event.entities.iter().for_each(|entity| {
                if let Ok((mut crafter)) = query.get_mut(*entity) {
                    let new_recipe = match crafter.recipe.as_ref() {
                        Some(recipe) => {
                            let new_recipe =
                                recipe.recipe_type.next_available_recipe(&available_recipes);
                            Some(new_recipe.into())
                        }
                        None => Some(Recipe::from(RecipeType::WoodToToy)),
                    };
                    match crafter.recipe.as_ref() {
                        Some(recipe)
                            if recipe.recipe_type
                                == new_recipe
                                    .as_ref()
                                    .map_or_else(|| panic!(), |recipe| recipe.recipe_type) =>
                        {
                            ()
                        }
                        _ => {
                            recipe_change_event.send(AssemblerRecipeChangedEvent(*entity));
                        }
                    }
                    crafter.recipe = new_recipe;
                    crafter.state = CrafterState::Pending(true);
                }
            });
        }
    }
}

fn handle_recipe_change(
    mut query: Query<(&mut Inventory, &Crafter), Without<Player>>,
    mut player: Query<&mut Inventory, With<Player>>,
    mut event: EventReader<AssemblerRecipeChangedEvent>,
) {
    let mut player_inventory = player.single_mut();
    for event in event.iter() {
        let (mut inventory, crafter) = match query.get_mut(event.0) {
            Ok((inventory, crafter)) => (inventory, crafter),
            Err(_) => continue,
        };
        inventory.force_empty_into_other(player_inventory.as_mut());
        if let Some(recipe) = crafter.recipe.as_ref() {
            inventory.filtered_for(Some(recipe));
        }
    }
}
