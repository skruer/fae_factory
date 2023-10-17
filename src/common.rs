use bevy::prelude::*;

#[derive(Component)]
struct MainMenuUI;

#[derive(Component)]
struct MyGameCamera;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Building;

#[derive(Component)]
struct Item {
    name: String,
    stack_size: u32,
}

#[derive(Component)]
struct Recipe {
    name: String,
    input: Vec<(Item, u32)>,  // Input cost
    output: Vec<(Item, u32)>, // Production
    cost: f32,                // Time to craft
}

#[derive(Component)]
struct Assembler {
    recipe: Recipe,
    progress: f32,
    progress_bar: Handle<ColorMaterial>,
    progress_bar_bg: Handle<ColorMaterial>,
}

#[derive(Component)]
struct Position(Vec2);

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct PlayerName(Name);
