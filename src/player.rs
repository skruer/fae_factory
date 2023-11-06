use std::collections::HashMap;

use crate::{
    assembler::{Assembler, AssemblerState},
    items::{Inventory, ItemId, ItemList},
    recipes::RecipeList,
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

#[derive(Component)]
pub struct Player;

fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    let texture = asset_server.load("character.png");

    commands.spawn((
        SpriteBundle {
            texture,
            ..default()
        },
        Player {},
        Speed(200.0),
        Name::new("Player"),
        Inventory::new(
            10,
            vec![
                (ItemId::new(ItemList::Wood), 10),
                (ItemId::new(ItemList::Crystal), 10),
            ],
        ),
        Assembler::new(),
    ));
}

fn player_movement_controls(
    mut characters: Query<(&mut Transform, &Speed, &Player)>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut transform, speed, _player) in &mut characters {
        let distance = speed.0 * time.delta_seconds();

        let keys = vec![
            (KeyCode::W, Vec2::new(0.0, 1.0)),
            (KeyCode::S, Vec2::new(0.0, -1.0)),
            (KeyCode::D, Vec2::new(1.0, 0.0)),
            (KeyCode::A, Vec2::new(-1.0, 0.0)),
        ];

        let direction = keys
            .iter()
            .map(|(key, direction)| match input.pressed(*key) {
                true => *direction,
                _ => Vec2::new(0.0, 0.0),
            })
            .fold(Vec2::new(0.0, 0.0), |acc, direction| acc + direction)
            .normalize();

        if direction.length() > 0.0 {
            transform.translation.x += distance * direction.x;
            transform.translation.y += distance * direction.y;
        }
    }
}

fn player_craft(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Inventory, &mut Assembler), With<Player>>,
) {
    for (mut inventory, mut assembler) in &mut player {
        if input.just_pressed(KeyCode::Space) {
            println!("Crafting!");
            assembler.recipe = Some(RecipeList::WoodToToy.get_recipe());
            assembler.state = AssemblerState::Pending;
        }
    }
}
