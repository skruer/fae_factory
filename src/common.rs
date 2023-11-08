use bevy::prelude::*;

use crate::{items::ItemId, recipes::Recipe, structures::StructureId};

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
pub struct BoundingBox(pub Rect);

#[derive(Component)]
pub struct Clickable;

#[derive(Component)]
pub struct Hoverable;

#[derive(Component)]
pub enum Holdable {
    Item(ItemId),
    Structure(StructureId),
}

#[derive(Component)]
pub struct Held(pub Option<Holdable>);

pub fn round_to_grid(pos: Vec2) -> Vec2 {
    let grid_step = 20.0;
    let round_up = |num: f32| -> f32 {
        let remainder = num % grid_step;
        if remainder == 0.0 {
            num
        } else {
            num + grid_step - remainder
        }
    };
    let x = round_up(pos.x.round()) as i32;
    let y = round_up(pos.y.round()) as i32;
    Vec2::new(x as f32, y as f32)
}
