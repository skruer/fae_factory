use bevy::prelude::*;

use crate::{items::ItemType, structures::StructureType};

struct FaeCommonPlugin;

impl Plugin for FaeCommonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameState>()
            .register_type::<MainMenuUI>()
            .register_type::<FaeGameCamera>()
            .register_type::<Speed>()
            .register_type::<Clickable>()
            .register_type::<Hoverable>()
            .register_type::<Held>()
            .register_type::<Holdable>();
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Reflect)]
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

#[derive(Component, Reflect)]
pub struct MainMenuUI;

#[derive(Component, Reflect)]
pub struct FaeGameCamera;

#[derive(Component, Reflect)]
pub struct Speed(f32);

#[derive(Component, Reflect, Default)]
pub struct Clickable;

#[derive(Component, Reflect, Default)]
pub struct Hoverable;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Facing {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Holdable {
    Item(ItemType),
    Structure(StructureType),
}

#[derive(Component, Reflect, Default)]
pub struct TimedProcessState {
    pub duration: f32,
    pub timer: Timer,
}

#[derive(Component, Reflect, Debug)]
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
