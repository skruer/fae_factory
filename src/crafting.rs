use bevy::prelude::*;

use crate::{items::inventory::Inventory, player::Player, recipes::Recipe};

pub struct CraftingPlugin;

impl Plugin for CraftingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_crafting)
            .add_systems(Update, cancel_player_crafting)
            .register_type::<Crafter>()
            .register_type::<Recipe>()
            .add_event::<CraftCompleteEvent>();
    }
}

#[derive(Event)]
pub struct CraftCompleteEvent {
    pub entity: Entity,
    pub recipe: Recipe,
}

#[derive(Component, Reflect, Default)]
pub struct Crafter {
    pub recipe: Option<Recipe>,
    pub progress: f32,
    pub state: CrafterState,
    //progress_bar: Handle<ColorMaterial>,
    //progress_bar_bg: Handle<ColorMaterial>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum CrafterState {
    Idle,
    Pending(bool),
    Assembling(bool),
}

impl Default for CrafterState {
    fn default() -> Self {
        CrafterState::Pending(true)
    }
}

#[derive(Component, Reflect)]
pub struct CrafterSpeed(pub f32);

impl Crafter {
    pub fn new() -> Self {
        Crafter {
            recipe: None,
            progress: 0.0,
            state: CrafterState::Idle,
            //progress_bar: asset_server.load("progress_bar.png"),
            //progress_bar_bg: asset_server.load("progress_bar_bg.png"),
        }
    }
}

fn handle_crafting(
    time: Res<Time>,
    mut assemblers: Query<(&mut Crafter, &mut Inventory, Option<&CrafterSpeed>, Entity)>,
    mut events: EventWriter<CraftCompleteEvent>,
) {
    for (mut assembler, mut inventory, speed, entity) in &mut assemblers {
        match assembler.state {
            CrafterState::Idle => (),
            CrafterState::Pending(repeating) => {
                if let Some(recipe) = &assembler.recipe {
                    if inventory.remove_items(&recipe.input) {
                        assembler.state = CrafterState::Assembling(repeating);
                    }
                }
            }
            CrafterState::Assembling(repeating) => {
                if let Some(recipe) = assembler.recipe.as_ref() {
                    let crafting_speed = speed.map_or(1.0, |s| s.0);
                    if assembler.progress + time.delta_seconds() * crafting_speed >= recipe.cost {
                        // Notify that crafting is complete
                        events.send(CraftCompleteEvent {
                            entity,
                            recipe: recipe.clone(),
                        });
                        // Update the items
                        inventory.add_items(&recipe.output);

                        assembler.state = match repeating {
                            true => CrafterState::Pending(true),
                            false => CrafterState::Idle,
                        };

                        assembler.progress = 0.0;
                    } else {
                        assembler.progress += time.delta_seconds();
                    }
                }
            }
        }
    }
}

fn cancel_player_crafting(
    input: Res<Input<KeyCode>>,
    mut assemblers: Query<(&mut Crafter, &mut Inventory), With<Player>>,
) {
    for (mut assembler, mut inventory) in &mut assemblers {
        if input.just_pressed(KeyCode::X) {
            if let Some(ref recipe) = &assembler.recipe {
                inventory.add_items(&recipe.input);
            }
            assembler.state = CrafterState::Idle;
            assembler.progress = 0.0;
            assembler.recipe = None;
        }
    }
}
