use bevy::prelude::*;

use super::FaeInputModifier;
use crate::{
    common::Clickable,
    map::grid::{GridPosition, HoveredGrid},
};

pub struct FaeMousePlugin;

#[derive(Event)]
pub struct FaeEntityClickEvent {
    pub entities: Vec<Entity>,
    pub modifiers: FaeInputModifier,
}

#[derive(Event)]
pub struct FaeEntityContextClickEvent {
    pub entities: Vec<Entity>,
    pub modifiers: FaeInputModifier,
}

impl Plugin for FaeMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_click)
            .add_event::<FaeEntityClickEvent>()
            .add_event::<FaeEntityContextClickEvent>();
    }
}

fn get_clicked_entities(
    mouse_grid: Res<HoveredGrid>,
    sprite_query: Query<(&GridPosition, &Transform, Entity), With<Clickable>>,
) -> Vec<Entity> {
    sprite_query
        .iter()
        .filter(|(grid, _, _)| **grid == mouse_grid.0)
        .map(|(_, _, entity)| entity)
        .collect()
}

fn handle_click(
    mouse_grid: Res<HoveredGrid>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(&GridPosition, &Transform, Entity), With<Clickable>>,
    mut left_click_writer: EventWriter<FaeEntityClickEvent>,
    mut right_click_writer: EventWriter<FaeEntityContextClickEvent>,
) {
    if mouse.any_just_pressed([MouseButton::Left, MouseButton::Right]) {
        let clicked = get_clicked_entities(mouse_grid, sprite_query);
        println!("Clicked: {:?}", clicked);

        if mouse.just_pressed(MouseButton::Left) {
            left_click_writer.send(FaeEntityClickEvent {
                entities: clicked.clone(),
                modifiers: FaeInputModifier::from(&keys),
            });
        }
        if mouse.just_pressed(MouseButton::Right) {
            right_click_writer.send(FaeEntityContextClickEvent {
                entities: clicked.clone(),
                modifiers: FaeInputModifier::from(&keys),
            });
        }
    }
}
