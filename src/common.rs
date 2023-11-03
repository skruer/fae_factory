use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    MainMenu,
    Playing,
    Paused,
}

impl Default for GameState {
    fn default() -> Self {
        GameState::Playing
    }
}

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub struct MyGameCamera;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(f32);

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct ItemType {
    id: String,
    stack_size: u32,
}

#[derive(Component)]
pub struct Recipe {
    name: String,
    input: Vec<(ItemType, u32)>,  // Input cost
    output: Vec<(ItemType, u32)>, // Production
    cost: f32,                    // Time to craft
}

#[derive(Component)]
pub struct Assembler {
    recipe: Option<Recipe>,
    progress: f32,
    //progress_bar: Handle<ColorMaterial>,
    //progress_bar_bg: Handle<ColorMaterial>,
}

#[derive(Component)]
pub struct Inventory {
    pub items: Vec<(ItemType, u32)>,
    pub slots: u32,
}

#[derive(Component)]
pub struct Position(Vec2);

#[derive(Component)]
pub struct PlayerName(Name);
