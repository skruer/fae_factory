use bevy::prelude::*;

use super::{FaeInputModifier, MyWorldCoords};
use crate::common::{BoundingBox, Clickable};

pub struct FaeMousePlugin;

#[derive(Event)]
pub struct FaeEntityClickEvent {
    pub entity: Option<Entity>,
    pub modifiers: FaeInputModifier,
}

#[derive(Event)]
pub struct FaeEntityContextClickEvent {
    pub entity: Option<Entity>,
    pub modifiers: FaeInputModifier,
}

impl Plugin for FaeMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (handle_lmb, handle_rmb))
            .add_event::<FaeEntityClickEvent>()
            .add_event::<FaeEntityContextClickEvent>();
    }
}

fn handle_lmb(
    cursor_position: Res<MyWorldCoords>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(&BoundingBox, &Transform, Entity), With<Clickable>>,
    mut click_writer: EventWriter<FaeEntityClickEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let clicked = sprite_query
            .iter()
            .filter(|(rect, _, _)| rect.0.contains(cursor_position.0))
            .max_by_key(|(_, transform, _)| transform.translation.z.round() as i32)
            .map(|(_, _, entity)| entity);

        println!("Clicked: {:?}", clicked);
        click_writer.send(FaeEntityClickEvent {
            entity: clicked,
            modifiers: FaeInputModifier::from(keys),
        });
    }
}

fn handle_rmb(
    cursor_position: Res<MyWorldCoords>,
    mouse: Res<Input<MouseButton>>,
    keys: Res<Input<KeyCode>>,
    sprite_query: Query<(&BoundingBox, &Transform, Entity), With<Clickable>>,
    mut click_writer: EventWriter<FaeEntityContextClickEvent>,
) {
    if mouse.just_pressed(MouseButton::Right) {
        let clicked = sprite_query
            .iter()
            .filter(|(rect, _, _)| rect.0.contains(cursor_position.0))
            .max_by_key(|(_, transform, _)| transform.translation.z.round() as i32)
            .map(|(_, _, entity)| entity);

        println!("Clicked: {:?}", clicked);
        click_writer.send(FaeEntityContextClickEvent {
            entity: clicked,
            modifiers: FaeInputModifier::from(keys),
        });
    }
}
