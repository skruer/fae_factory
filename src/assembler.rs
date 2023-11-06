use bevy::prelude::*;

use crate::{items::Inventory, player::Player, recipes::Recipe};

pub struct AssemblerPlugin;

impl Plugin for AssemblerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_assembling)
            .add_systems(Update, cancel_assembling)
            .register_type::<Assembler>()
            .register_type::<Recipe>();
    }
}

#[derive(Component, Reflect)]
pub struct Assembler {
    pub recipe: Option<Recipe>,
    pub progress: f32,
    pub state: AssemblerState,
    //progress_bar: Handle<ColorMaterial>,
    //progress_bar_bg: Handle<ColorMaterial>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
pub enum AssemblerState {
    Idle,
    Pending,
    Assembling,
    Repeating,
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            recipe: None,
            progress: 0.0,
            state: AssemblerState::Idle,
            //progress_bar: asset_server.load("progress_bar.png"),
            //progress_bar_bg: asset_server.load("progress_bar_bg.png"),
        }
    }
}

fn handle_assembling(time: Res<Time>, mut assemblers: Query<(&mut Assembler, &mut Inventory)>) {
    for (mut assembler, mut inventory) in &mut assemblers {
        match assembler.state {
            AssemblerState::Idle => (),
            AssemblerState::Pending => {
                if let Some(recipe) = &assembler.recipe {
                    let mut can_craft = true;
                    for (item, amount) in &recipe.input {
                        if !inventory.has_item(item, *amount) {
                            can_craft = false;
                            break;
                        }
                    }
                    if can_craft {
                        for (item, amount) in &recipe.input {
                            inventory.remove_item(item, *amount);
                        }
                        assembler.state = AssemblerState::Assembling;
                    }
                }
            }
            AssemblerState::Assembling | AssemblerState::Repeating => {
                if let Some(recipe) = assembler.recipe.as_ref() {
                    if assembler.progress + time.delta_seconds() >= recipe.cost {
                        for (item, amount) in &recipe.output {
                            inventory.add_item(item, *amount);
                        }
                        // Either repeat or stop
                        assembler.state = match assembler.state {
                            AssemblerState::Assembling => AssemblerState::Idle,
                            AssemblerState::Repeating => AssemblerState::Pending,
                            _ => unreachable!(),
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

fn cancel_assembling(
    input: Res<Input<KeyCode>>,
    mut assemblers: Query<(&mut Assembler, &mut Inventory), With<Player>>,
) {
    for (mut assembler, mut inventory) in &mut assemblers {
        if input.just_pressed(KeyCode::X) {
            if let Some(ref recipe) = &assembler.recipe {
                for (item, amount) in &recipe.input {
                    inventory.add_item(item, *amount);
                }
            }
            assembler.state = AssemblerState::Idle;
            assembler.progress = 0.0;
            assembler.recipe = None;
        }
    }
}
