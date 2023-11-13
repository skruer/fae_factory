use crate::{
    common::Held,
    crafting::{Crafter, CrafterState},
    items::{inventory::Inventory, ItemType},
    map::grid::GridPosition,
    recipes::{Recipe, RecipeType},
    Speed,
};

use bevy::prelude::*;

use self::events::PlayerMoveEvent;

pub mod events;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (player_movement_controls, player_craft))
            .add_event::<PlayerMoveEvent>();
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
    pub grid_position: GridPosition,
    pub speed: Speed,
    pub inventory: Inventory,
    pub crafter: Crafter,
    pub held: Held,
}

impl Default for FaePlayerBundle {
    fn default() -> Self {
        use ItemType::*;
        FaePlayerBundle {
            player: Player,
            player_move: PlayerMove(None),
            speed: Speed(200.0),
            inventory: Inventory::new(
                10,
                Some(vec![(Wood, 10), (Crystal, 10), (Stone, 10), (Toy, 10)]),
            ),
            crafter: Crafter::new(),
            held: Held(None),
            grid_position: GridPosition::default(),
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
    mut player: Query<(&Speed, &mut Transform, &mut GridPosition), With<Player>>,
    time: Res<Time>,
    mut event: EventReader<PlayerMoveEvent>,
) {
    if let Some(direction) = event.iter().last() {
        let direction = direction.0;
        let (speed, mut transform, mut grid_position) = player.single_mut();
        let distance = speed.0 * time.delta_seconds();

        if direction.length() > 0.0 {
            transform.translation.x += distance * direction.x;
            transform.translation.y += distance * direction.y;
            *grid_position = GridPosition::from_translation(transform.translation);
        }
    }
}

fn player_craft(
    mut commands: Commands,
    input: Res<Input<KeyCode>>,
    mut player: Query<&mut Crafter, With<Player>>,
) {
    for (mut assembler) in &mut player {
        if input.just_pressed(KeyCode::Space) && assembler.state == CrafterState::Idle {
            println!("Crafting!");
            assembler.recipe = Some(Recipe::from(RecipeType::WoodToToy));
            assembler.state = CrafterState::Pending(false); // Don't repeat crafting for the player
        }
    }
}
