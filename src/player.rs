use std::collections::HashMap;

use crate::{
    crafting::{AssemblerState, Crafter},
    items::{Inventory, ItemId, ItemList},
    recipes::RecipeList,
    structures::SelectedStructure,
    Speed,
};

use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement_controls, player_craft));
    }
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Component, Reflect)]
pub struct PlayerMove(pub Option<Vec2>);

#[derive(Bundle)]
pub struct FaePlayerBundle {
    pub player: Player,
    pub player_move: PlayerMove,
    pub speed: Speed,
    pub inventory: Inventory,
    pub crafter: Crafter,
    pub selected_structure: SelectedStructure,
}

impl Default for FaePlayerBundle {
    fn default() -> Self {
        FaePlayerBundle {
            player: Player,
            player_move: PlayerMove(None),
            speed: Speed(200.0),
            inventory: Inventory::new(
                10,
                vec![
                    (ItemId::new(ItemList::Wood), 10),
                    (ItemId::new(ItemList::Crystal), 10),
                ],
            ),
            crafter: Crafter::new(),
            selected_structure: SelectedStructure(None),
        }
    }
}

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        FaePlayerBundle { ..default() },
    ));
}

fn player_movement_controls(
    mut player: Query<(&PlayerMove, &Speed, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    let (direction, speed, mut transform) = player.single_mut();
    if direction.0.is_none() {
        return;
    }

    let direction = direction.0.unwrap();

    let distance = speed.0 * time.delta_seconds();

    if direction.length() > 0.0 {
        transform.translation.x += distance * direction.x;
        transform.translation.y += distance * direction.y;
    }
}

fn player_craft(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Inventory, &mut Crafter), With<Player>>,
) {
    for (mut inventory, mut assembler) in &mut player {
        if input.just_pressed(KeyCode::Space) && assembler.state == AssemblerState::Idle {
            println!("Crafting!");
            assembler.recipe = Some(RecipeList::WoodToToy.get_recipe());
            assembler.state = AssemblerState::Pending(false); // Don't repeat crafting for the player
        }
    }
}
