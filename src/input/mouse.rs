use bevy::prelude::*;

use super::MyWorldCoords;
use crate::common::{BoundingBox, Clickable};

pub struct FaeMousePlugin;

#[derive(Event)]
pub struct FaeEntityClickEvent(Option<Entity>);

impl Plugin for FaeMousePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_lmb)
            .add_event::<FaeEntityClickEvent>();
    }
}

fn handle_lmb(
    cursor_position: Res<MyWorldCoords>,
    mouse: Res<Input<MouseButton>>,
    sprite_query: Query<(&BoundingBox, &Transform, Entity), With<Clickable>>,
    mut click_writer: EventWriter<FaeEntityClickEvent>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let clicked = sprite_query
            .iter()
            .filter(|(rect, _, _)| rect.0.contains(cursor_position.0))
            .max_by_key(|(_, transform, _)| transform.translation.z.round() as i32)
            .map(|(_, _, entity)| entity);

        click_writer.send(FaeEntityClickEvent(clicked));
    }
}
