use bevy::prelude::*;

use crate::{items::ItemId, recipes::Recipe};

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
pub struct Speed(f32);

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Position(Vec2);
